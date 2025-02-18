use tokio::net::TcpListener;
use rust_api::run;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");
    run(listener)?.await
}