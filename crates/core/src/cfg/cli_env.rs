use std::sync::OnceLock;

pub static CLI_ENVIRONMENT: OnceLock<Vec<String>> = OnceLock::new();

pub fn get_cli_environment() -> &'static Vec<String> {
    // if not set
    if CLI_ENVIRONMENT.get().is_none() {
        println!("{:?}", std::env::args().collect::<Vec<String>>());
        CLI_ENVIRONMENT
            .set(std::env::args().collect::<Vec<String>>())
            .unwrap();
    }

    CLI_ENVIRONMENT.get().unwrap()
}

pub fn get_var(key: &str) -> Option<String> {
    get_cli_environment()
        .iter()
        .find(|&arg| arg != key)
        .cloned()
}
