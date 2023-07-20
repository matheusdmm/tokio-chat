use std::io;
use std::io::Write;
use tokio::io::AsyncBufReadExt;
use tokio::time;
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, io::BufReader, net::TcpListener, net::TcpStream};

#[allow(dead_code)]
async fn run() {
    println!("Sleeeep");
}

#[allow(dead_code, unused)]
async fn run2() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input);

    time::sleep(time::Duration::from_secs(2)).await;
    println!("{} Wooooke", user_input);
}

#[allow(dead_code)]
async fn run3() {
    println!("Hello, world!");
    run().await;
    run2().await;
}

#[tokio::main]
async fn main() {
    const HOST: &str = "localhost:8080";
    let listener = TcpListener::bind(HOST).await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            //handle_connection(socket).await;
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();

                if bytes_read == 0 {
                    break;
                }

                //socket.write_all(&line.as_bytes()).await.unwrap();
                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
