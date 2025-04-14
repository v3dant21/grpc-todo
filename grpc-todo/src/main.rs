use server::run_server;
use dotenv::dotenv;

mod config;
mod build;
mod db;
mod server;
#[tokio::main]
async fn main() {
    run_server().await;
    dotenv().ok();
}
