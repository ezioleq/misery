use clap::Parser;
use config::Config;
use log::debug;

mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::parse();
    debug!("Arguments: {:?}", config);

    server::start_server().await;
}
