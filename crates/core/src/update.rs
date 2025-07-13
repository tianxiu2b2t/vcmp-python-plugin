use std::{
    cell::LazyCell,
    thread,
    time::{Duration, Instant},
};

use toml;
use tracing::{Level, event};
use ureq;

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub version: String,
    pub url: String,
}

pub const INFO: LazyCell<PluginInfo> = LazyCell::new(|| {
    let pyproject_content = include_str!("../../../pyproject.toml");
    let pyproject = toml::from_str::<toml::Table>(pyproject_content).unwrap();
    let version = pyproject
        .get("project")
        .unwrap()
        .get("version")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let repo_url = pyproject
        .get("project")
        .unwrap()
        .get("urls")
        .unwrap()
        .get("repository")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    PluginInfo {
        version,
        url: repo_url,
    }
});

pub fn get_repo() -> String {
    // https://github.com/author/reop/
    let url = INFO.url.clone();
    url.replace("https://github.com/", "")
        .split("/")
        .take(2)
        .collect::<Vec<&str>>()
        .join("/")
        .to_string()
}

pub fn check() {
    let repo = get_repo();
    let session = ureq::get(format!(
        "https://api.github.com/repos/{repo}/releases/latest"
    ))
    .config()
    .user_agent(format!("VCMP-Python-Plugin-Checker/{}", INFO.version))
    .build()
    .call();
    if let Err(e) = session {
        event!(Level::ERROR, "Failed to check for updates: {}", e);
        return;
    }

    let binding = session.unwrap().into_body().read_to_string().unwrap();
    let tag_name = binding
        .split("\"tag_name\":\"v")
        .nth(1)
        .unwrap()
        .split("\"")
        .nth(0)
        .unwrap();
    if tag_name != INFO.version {
        event!(
            Level::INFO,
            "New version available: {}, current version: {}",
            tag_name,
            INFO.version
        );
    }
}

pub fn init() {
    // 开线程，防止阻塞
    let _ = thread::spawn(|| {
        loop {
            let start_time = Instant::now();
            check();
            thread::sleep(Duration::from_secs(86400) - start_time.elapsed());
        }
    });
}
