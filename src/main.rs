mod common;
mod config;
mod http_server;
mod meta;
mod metrics;
mod nut_client;
mod openmetrics_builder;

#[tokio::main]
async fn main() {
    let config = config::read_config();
    if config.print_metrics_and_exit {
        metrics::print_metrics()
    } else {
        http_server::run_server(config).await;
    }
}
