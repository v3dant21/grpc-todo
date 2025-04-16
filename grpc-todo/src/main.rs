use server::run_server;
use dotenv::dotenv;

mod config;
mod db;
mod server;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_server().await;
    dotenv().ok();
}
