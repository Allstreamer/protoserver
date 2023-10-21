use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpStream, TcpListener},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
enum Methods {
    #[serde(rename = "isPrime")]
    IsPrime,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize)]
struct Request {
    method: Methods,
    number: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
struct Response {
    method: Methods,
    prime: bool,
}

static EMPTY: &'static str = "{}";

#[allow(dead_code)]
pub async fn prime_time(listener: TcpListener) {
    loop {
        match listener.accept().await {
            Ok((socket, ip)) => {
                println!("Connection from: {:?}", ip);
                tokio::spawn(async move {
                    run(socket).await;
                });
            }
            _ => {}
        }
    }
}

async fn run(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    loop {
        let mut line = String::new();
        if let Err(e) = reader.read_line(&mut line).await {
            println!("{:?} Failed to read", e);
            reader.write(&EMPTY.as_bytes()).await.unwrap();
            reader.flush().await.unwrap();
            reader.shutdown().await.unwrap();
            return;
        }

        if line.is_empty() {
            reader.write(&EMPTY.as_bytes()).await.unwrap();
            reader.flush().await.unwrap();
            reader.shutdown().await.unwrap();
            return;
        }
        println!("{}", line);

        let data = serde_json::from_str(&line);

        if let Err(e) = data {
            println!("{:?} Failed to parse json", e);
            reader.write(&EMPTY.as_bytes()).await.unwrap();
            reader.flush().await.unwrap();
            reader.shutdown().await.unwrap();
            return;
        }
        let data: Request = data.unwrap();

        let is_prime = if data.number.is_sign_negative() {
            false
        } else {
            primal::is_prime(data.number as u64)
        };

        let mut response_data = serde_json::to_string(&Response {
            method: data.method,
            prime: is_prime,
        })
        .unwrap();
        response_data.push('\n');
        println!("{}", response_data);
        reader.write(response_data.as_bytes()).await.unwrap();
        reader.flush().await.unwrap();
    }
}
