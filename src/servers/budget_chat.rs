use std::sync::Arc;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    select,
    sync::broadcast::{self, Receiver, Sender},
    sync::Mutex,
};

static WELCOME_MESSAGE: &str = "Welcome to ProtoChat! Enter your name:\n";

#[derive(Debug, Clone)]
enum Command {
    Join(String),
    Message { name: String, msg: String },
    Leave(String),
}

#[allow(dead_code)]
pub async fn budget_chat(listener: TcpListener) {
    let (tx, _) = broadcast::channel(1000);
    let users: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    loop {
        match listener.accept().await {
            Ok((socket, ip)) => {
                let tx_clone = tx.clone();
                let rx_clone = tx.subscribe();
                let users = users.clone();
                println!("Connection from: {:?}", ip);
                tokio::spawn(async move {
                    run(socket, tx_clone, rx_clone, users).await;
                });
            }
            _ => {}
        }
    }
}

async fn run(
    socket: TcpStream,
    tx: Sender<Command>,
    mut rx: Receiver<Command>,
    users: Arc<Mutex<Vec<String>>>,
) {
    let mut reader = BufReader::new(socket);
    reader.write(WELCOME_MESSAGE.as_bytes()).await.unwrap();

    let mut username = String::new();
    let mut logged_in = false;
    loop {
        let mut buf = String::new();

        select! {
            _ = reader.
            _ = reader.read_line(&mut buf) => {
                match logged_in {
                    false => {
                        username = String::from(buf.trim());
                        logged_in = true;

                        let mut users_lock = users.lock().await;
                        let user_str_list = format!("{:?}", users_lock).replace("[", "").replace("]", "").replace("\"", "");
                        reader.write(format!("* The room contains: {}\n", user_str_list).as_bytes()).await.unwrap();
                        users_lock.push(username.to_owned());

                        if let Err(e) = tx.send(Command::Join(username.to_owned())) {
                            println!("{:?}", e);
                            break;
                        }
                    },
                    true => {
                        if let Err(e) = tx.send(Command::Message{name: username.to_owned(), msg: buf}) {
                            println!("{:?}", e);
                            break;
                        }
                    }
                }
            }
            val = rx.recv() => {
                if let Err(e) = val {
                    println!("{:?}", e);
                    break;
                }
                let cmd = val.unwrap();
                match cmd {
                    Command::Join(v) => {
                        if v != username {
                            reader.write(format!("* {} has entered the room\n",v).as_bytes()).await.unwrap();
                        }
                    },
                    Command::Message { name, msg } => {
                        if name != username {
                            if let Err(e) = reader.write(format!("[{}] {}\n", name, msg.trim()).as_bytes()).await {
                                println!("{:?}", e);
                                break;
                            }
                        }
                    },
                    Command::Leave(v) => {if v != username {let _ = reader.write(format!("* {} has left the room\n", v).as_bytes()).await;}},
                };
            }
        }
    }
    tx.send(Command::Leave(username.to_owned())).unwrap();
    let mut user_lock = users.lock().await;
    println!("{:?}", user_lock);
    let index = user_lock.iter().position(|x| *x == username).unwrap();
    user_lock.remove(index);
}
