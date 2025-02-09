#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
