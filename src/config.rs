use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub http_port: u16,
    pub http_path: String,
    pub log_requests_console: bool,
    pub print_metrics_and_exit: bool,
}

impl Config {
    const DEFAULT_HTTP_PORT: u16 = 9995;
    const DEFAULT_HTTP_PATH: &'static str = "/nut";
    const DEFAULT_LOG_REQUESTS_CONSOLE: bool = false;
    const DEFAULT_PRINT_METRICS_AND_EXIT: bool = false;
}

pub fn read_config() -> Config {
    let mut config = Config {
        http_port: Config::DEFAULT_HTTP_PORT,
        http_path: Config::DEFAULT_HTTP_PATH.to_owned(),
        log_requests_console: Config::DEFAULT_LOG_REQUESTS_CONSOLE,
        print_metrics_and_exit: Config::DEFAULT_PRINT_METRICS_AND_EXIT,
    };

    if let Ok(http_port_str) = env::var("HTTP_PORT") {
        if let Ok(http_port) = http_port_str.parse::<u16>() {
            config.http_port = http_port;
        }
    }
    if let Ok(http_path) = env::var("HTTP_PATH") {
        if http_path.starts_with('/') {
            config.http_path = http_path;
        }
    }
    if let Ok(log_requests_console_str) = env::var("LOG_REQUESTS_CONSOLE") {
        if let Ok(log_requests_console) = log_requests_console_str.parse::<bool>() {
            config.log_requests_console = log_requests_console;
        }
    }
    if let Ok(print_metrics_and_exit_str) = env::var("PRINT_METRICS_AND_EXIT") {
        if let Ok(print_metrics_and_exit) = print_metrics_and_exit_str.parse::<bool>() {
            config.print_metrics_and_exit = print_metrics_and_exit;
        }
    }

    config
}
