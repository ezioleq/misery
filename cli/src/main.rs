#[tokio::main]
async fn main() {
    env_logger::init();

    server::start_server().await;
}
