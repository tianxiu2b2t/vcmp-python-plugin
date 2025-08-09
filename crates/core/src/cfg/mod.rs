use std::{
    fmt::{Display, Formatter},
    path::Path,
    str::FromStr,
    sync::OnceLock,
};

use tracing::{Level, event};

pub mod cli_env;

#[derive(Debug, Clone)]
pub struct Config {
    pub preloader: bool,     // 直接在 VcmpPluginInit 时候加载
    pub script_path: String, // 脚本路径
    pub virtual_env: String, // 虚拟环境路径 (建议是包)
    pub log_level: Level,    // 日志等级
    pub check_update: bool,
    pub file_log: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            preloader: false,
            script_path: "".to_string(),
            virtual_env: "".to_string(),
            log_level: Level::INFO,
            check_update: true,
            file_log: false,
        }
    }

    pub fn set_preloader(&mut self, preloader: bool) {
        self.preloader = preloader;
    }

    pub fn set_script_path(&mut self, script_path: String) {
        self.script_path = script_path;
    }

    pub fn set_virtual_env(&mut self, virtual_env: String) {
        self.virtual_env = virtual_env;
    }
    pub fn set_log_level(&mut self, log_level: Level) {
        self.log_level = log_level;
    }
    pub fn set_check_update(&mut self, check_update: bool) {
        self.check_update = check_update;
    }
    pub fn set_file_log(&mut self, file_log: bool) {
        self.file_log = file_log;
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Config {{
            preloader: {},
            script_path: "{}",
            virtual_env: {:?},
            log_level: {:?},
            check_update: {},
            file_log: {},
        }}
        "#,
            self.preloader,
            self.script_path,
            self.virtual_env,
            self.log_level,
            self.check_update,
            self.file_log
        )
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

// support cfg and toml
// 优先 toml
// secondary cfg

fn init_config_from_cfg() -> Option<Config> {
    // default server.cfg
    let mut config = Config::new(); // dev... toml

    let cfg_file = Path::new("./server.cfg");
    if !cfg_file.exists() || !cfg_file.is_file() || cfg_file.metadata().expect("Failed to get metadata").len() == 0 {
        return None;
    }

    let content = std::fs::read_to_string(cfg_file).expect("Failed to read server.cfg");

    let find_value = |key: &str| {
        let mut value = String::new();
        for line in content.lines() {
            if line.starts_with(key) {
                value = line.split(' ').nth(1).expect("Failed to split line").trim().to_string();
                break;
            }
        }
        value
    };

    config.preloader = find_value("python_preloader").parse().unwrap_or(false);
    config.script_path = find_value("python_script_path").to_string();
    config.virtual_env = find_value("python_virtual_env").to_string();
    config.log_level =
        Level::from_str(find_value("python_log_level").as_str()).unwrap_or(Level::INFO);
    config.check_update = find_value("python_check_update").parse().unwrap_or(true);
    config.file_log = find_value("python_file_log").parse().unwrap_or(false);

    Some(config)
}

fn init_config_from_toml() -> Option<Config> {
    None
}

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("config not init")
}

pub fn init_config() {
    CONFIG.get_or_init(|| {
        init_config_from_toml().unwrap_or(init_config_from_cfg().unwrap_or_default())
    });

    event!(Level::DEBUG, "{}", get_config());
}
