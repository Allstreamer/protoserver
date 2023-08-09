use std::io;

use tokio::{net::{TcpStream, TcpListener}, io::Interest};

use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:5000").await?;

    loop {
        let (socket, _): (TcpStream, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(stream: TcpStream) {

    loop {
        let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await.unwrap();

        let mut echo_data = vec![];
        if ready.is_readable() {
            let mut read_buf = vec![0; 1024];
            match stream.try_read(&mut read_buf) {
                Ok(n) => {
                    if n == 0 {
                        continue;
                    }
                    echo_data.extend_from_slice(&read_buf[0..n]);
                    println!("{:?}", echo_data);
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                },
                Err(e) => {
                    panic!("Ono Failed Read! {:?}", e);
                },

            }
        }

        if ready.is_writable() {
            match stream.try_write(&echo_data) {
                Ok(_) => {
                
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue
                },
                Err(e) => {
                    panic!("Ono Failed Read! {:?}", e);
                }
            }
        }
    }
}
