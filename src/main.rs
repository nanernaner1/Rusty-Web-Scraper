use reqwest::Client;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let start = Instant::now();

    let futures = (0..100).map(|_| async {
        client.get("https://www.example.com/").send().await?.text().await
    });

    let documents: Vec<String> = futures::future::try_join_all(futures).await?;
    let elapsed = start.elapsed();

    println!("Fetched and stored {} documents", documents.len());
    println!("Time taken to fetch and parse 100 pages: {:?}", elapsed);

    Ok(())
}
