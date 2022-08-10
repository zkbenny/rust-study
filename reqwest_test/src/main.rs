#[tokio::main]
async fn main() {
    let blocking_client = reqwest::blocking::Client::new();
    println!("Hello, world: {:?}", blocking_client);
}
