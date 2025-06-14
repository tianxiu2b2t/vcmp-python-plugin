use std::{
    fmt::{Display, Formatter},
    sync::OnceLock,
};

pub mod cli_env;

pub struct Config {
    pub preloader: bool,     // 直接在 VcmpPluginInit 时候加载
    pub script_path: String, // 脚本路径
    pub virtual_env: String, // 虚拟环境路径 (建议是包)
    pub debug: bool,         // 是否调试
}

impl Config {
    pub fn new() -> Self {
        Self {
            preloader: false,
            script_path: "".to_string(),
            virtual_env: "".to_string(),
            debug: false,
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
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
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
            debug: {}
        }}
        "#,
            self.preloader,
            self.script_path,
            self.virtual_env.clone(),
            self.debug
        )
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config() {
    let mut config = Config::new();
    config = Config::new();
    config.set_script_path("./main.py".to_string());
    config.set_virtual_env("../.venv/Lib/site-packages".to_string());
    CONFIG.get_or_init(|| config);

    println!("{}", CONFIG.get().unwrap());
}
