use tokio::time;


async fn run() {
    println!("Sleeeep");
    time::sleep(time::Duration::from_secs(1)).await;
    println!("Wooooke");
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    run().await;
}
