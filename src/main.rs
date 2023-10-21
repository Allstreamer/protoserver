use tokio::net::TcpListener;

mod servers;
use servers::*;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    budget_chat(listener).await;
}
