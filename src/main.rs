use tokio::io::AsyncBufReadExt;
use tokio::sync::broadcast;
use tokio::{io::AsyncWriteExt, io::BufReader, net::TcpListener};

#[tokio::main]
async fn main() {
    const HOST: &str = "localhost:8080";
    const CAPACITY: usize = 10;
    let listener = TcpListener::bind(HOST).await.unwrap();
    let (tx, rx) = broadcast::channel(CAPACITY);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }

                    }
                }
            }
        });
    }
}
