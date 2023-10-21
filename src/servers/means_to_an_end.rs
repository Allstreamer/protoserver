use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, TcpListener},
};

#[allow(dead_code)]
pub async fn means_to_an_end(listener: TcpListener) {
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
    let mut buf = [0; 9];
    let mut data_vec: Vec<(i64, i64)> = vec![];

    loop {
        let n = socket.read_exact(&mut buf).await.unwrap();
        if n == 0 {
            return;
        }
        /*
        for &num in buf.iter() {
            print!("{:02X} ", num);
        }
        print!("\n");
        */
        let first_num = i32::from_be_bytes([buf[1],buf[2],buf[3],buf[4]]) as i64;
        let second_num = i32::from_be_bytes([buf[5],buf[6],buf[7],buf[8]]) as i64;
        match buf[0].try_into() {
            Ok('I') => {
                println!("I{} {}", first_num, second_num);
                data_vec.push((first_num, second_num));
            },
            Ok('Q') => {
                println!("Q{} {}", first_num, second_num);

                let mut vec_to_avr = vec![];
                data_vec.iter().for_each(|x| {
                    if x.0 >= first_num && x.0 <= second_num {
                        vec_to_avr.push(x.1);
                    }
                });

                let sum = vec_to_avr.iter().sum::<i64>();
                let len = vec_to_avr.len() as i64;

                let avg: i64 = if sum != 0 && len != 0 {sum / len} else {0};
                println!("A:{}", avg);
                socket.write(&(avg as i32).to_be_bytes()).await.unwrap();
            },
            _ => {
                continue;
            }
        }
   }
}
