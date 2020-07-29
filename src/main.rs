mod lib;

#[tokio::main]
pub async fn main() {
    lib::upload().await;
}
