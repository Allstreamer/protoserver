use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, TcpListener},
};

#[allow(dead_code)]
pub async fn smoke_test(listener: TcpListener) {
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

async fn run(mut socket: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                return;
            }
            Ok(n) => {
                socket.write_all(&buf[0..n]).await.unwrap();
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}

