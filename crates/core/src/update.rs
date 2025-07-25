use std::{
    sync::OnceLock,
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

// 使用标准库的OnceLock实现线程安全的延迟初始化
pub static INFO: OnceLock<PluginInfo> = OnceLock::new();

// 辅助函数：获取PluginInfo实例（首次调用时初始化）
fn get_info() -> &'static PluginInfo {
    INFO.get_or_init(|| {
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
    })
}

pub fn get_repo() -> String {
    let info = get_info();
    let url = info.url.clone();
    url.replace("https://github.com/", "")
        .split("/")
        .take(2)
        .collect::<Vec<&str>>()
        .join("/")
        .to_string()
}

pub fn check() {
    let repo = get_repo();
    let info = get_info();

    let session = ureq::get(format!(
        "https://api.github.com/repos/{repo}/releases/latest"
    ))
    .config()
    .user_agent(format!("VCMP-Python-Plugin-Checker/{}", info.version))
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
        .next()
        .unwrap();

    if tag_name != info.version {
        event!(
            Level::INFO,
            "New version available: {}, current version: {}",
            tag_name,
            info.version
        );
    }
}

pub fn init() {
    // 现在可以安全地在多线程中使用INFO了
    let _ = thread::spawn(|| {
        loop {
            let start_time = Instant::now();
            check();
            thread::sleep(Duration::from_secs(86400) - start_time.elapsed());
        }
    });
}
