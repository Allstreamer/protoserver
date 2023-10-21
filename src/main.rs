use tokio::net::TcpListener;

mod servers;
use servers::*;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((socket, ip)) => {
                println!("Connection from: {:?}", ip);
                tokio::spawn(async move {
                    prime_time(socket).await;
                });
            },
            _ => {},
        }
    }
}