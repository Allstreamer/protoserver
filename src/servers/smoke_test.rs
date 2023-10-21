use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn smoke_test(mut socket: TcpStream) {
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