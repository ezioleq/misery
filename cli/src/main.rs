use clap::Parser;
use config::Config;
use log::debug;

mod config;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let config = Config::parse();
    debug!("Arguments: {:?}", config);

    server::start_server().await;
}
