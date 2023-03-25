use ethers_providers::{Middleware, Provider, StreamExt, Ws};

#[tokio::main]
async fn main() {
    // NOTE: had to use with my api key bc WS is not free at Ankr.
    let wss: Provider<Ws> = Provider::<Ws>::connect("wss://rpc.ankr.com/eth/ws/aae39f307a37caf690e383fc1bb0107d8f126c41bd1c6324fd4bd7d72dab5a0f")
        .await
        .unwrap();

    let mut sub = wss.subscribe_blocks().await.unwrap();
    while let Some(block) = sub.next().await {
        println!("{:?}", block.number);
    }
}
