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
