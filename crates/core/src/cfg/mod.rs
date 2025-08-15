/*use std::{
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
    pub ignore_modules: Vec<String>,
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
            ignore_modules: vec![],
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
    pub fn set_ignore_modules(&mut self, ignore_modules: Vec<String>) {
        self.ignore_modules = ignore_modules;
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
            ignore_modules: {:?}
        }}
        "#,
            self.preloader,
            self.script_path,
            self.virtual_env,
            self.log_level,
            self.check_update,
            self.file_log,
            self.ignore_modules
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
*/

use std::{collections::HashMap, path::Path, sync::OnceLock};

use serde::Deserialize;
use tracing::Level;
use vcmp_bindings::encodes::decode_gbk;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ScriptConfig {
    pub script_path: String,
    pub virtual_env: String,
    pub preloader: bool,
}

#[derive(Debug, Clone)]
pub struct LogLevel(Level);

impl LogLevel {
    pub fn as_level(&self) -> Level {
        self.0
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.to_lowercase();
        match s.as_str() {
            "error" => Self(Level::ERROR),
            "warn" => Self(Level::WARN),
            "info" => Self(Level::INFO),
            "debug" => Self(Level::DEBUG),
            "trace" => Self(Level::TRACE),
            _ => Self(Level::INFO)
        }
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let t = String::deserialize(deserializer)?.to_lowercase();
        let s = t.as_str();
        Ok(Self::from_str(s))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggerConfig {
    pub log_level: LogLevel,
    pub file_log: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_level: LogLevel(Level::INFO),
            file_log: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdvancedConfig {
    pub check_update: bool,
    pub ignore_py_modules: Vec<String>,
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            check_update: true,
            ignore_py_modules: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    pub script: ScriptConfig,
    pub logger: LoggerConfig,
    pub advanced: AdvancedConfig,
}

fn init_config_from_toml() -> Option<Config> {
    // default: python_cfg.toml
    let content = match std::fs::read_to_string("./python_cfg.toml") {
        Ok(content) => content,
        Err(_) => return None,
    };

    match toml::from_str::<Config>(&content) {
        Ok(config) => Some(config),
        Err(e) => {
            println!("Failed to parse toml config: {e}");
            None
        },
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

fn parse_bool(s: &String) -> bool {
    let s = s.to_lowercase();
    match s.as_str() {
        "true" | "t" | "1" | "yes" | "y" | "on" | "enable" => true,
        _ => false
    }
}

fn init_config_from_cfg() -> Config {
    // default server.cfg
    let mut config = Config::default(); // dev... toml
    
    let cfg_file = Path::new("./server.cfg");
    
    // let config maybe ascii (like gbk)
    let content = {
        let c = std::fs::read(cfg_file).expect("Failed to read server.cfg");
        decode_gbk(&c)
    };

    let dictionary = {
        let mut dictionary: HashMap<String, String> = HashMap::new();
        for line in content.lines() {
            // split once " "
            let mut iter = line.splitn(2, " ");
            let key = iter.next().unwrap_or("").to_string();
            let value = iter.next().unwrap_or("").to_string();
            dictionary.insert(key, value);
        }
        dictionary
    };

    config.script.preloader = parse_bool(dictionary.get("python_preloader").unwrap_or(&"0".to_string()));
    config.script.script_path = dictionary.get("python_script_path").unwrap_or(&"".to_string()).to_string();
    config.script.virtual_env = dictionary.get("python_virtual_env").unwrap_or(&"".to_string()).to_string();

    config.logger.log_level = LogLevel::from_str(dictionary.get("python_log_level").unwrap_or(&"info".to_string()));
    config.logger.file_log = parse_bool(dictionary.get("python_file_log").unwrap_or(&"0".to_string()));
    config.advanced.check_update = parse_bool(dictionary.get("python_check_update").unwrap_or(&"1".to_string()));
    config.advanced.ignore_py_modules = dictionary.get("python_ignore_py_modules").unwrap_or(&"".to_string()).split(',').map(|s| s.to_string()).collect();

    config


}

pub fn init_config() {
    CONFIG.set(init_config_from_toml().unwrap_or_else(|| init_config_from_cfg())).unwrap();
}

pub fn get_config() -> &'static Config {
    CONFIG.get().unwrap()
}

pub fn get_script_path() -> String {
    get_config().script.script_path.clone()
}
pub fn get_virtual_env() -> String {
    get_config().script.virtual_env.clone()
}
pub fn get_preloader() -> bool {
    get_config().script.preloader
}
pub fn get_log_level() -> Level {
    get_config().logger.log_level.as_level()
}
pub fn get_file_log() -> bool {
    get_config().logger.file_log
}
pub fn get_check_update() -> bool {
    get_config().advanced.check_update
}
pub fn get_ignore_py_modules() -> Vec<String> {
    get_config().advanced.ignore_py_modules.clone()
}
