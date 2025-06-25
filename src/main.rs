use reqwest::Client;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let start = Instant::now();

    // Example URLs to fetch
    // You can replace these with any URLs you want to test
    // Here we use two URLs as an example, but you can add more
    // to the `urls` vector to simulate fetching multiple pages.
    let urls: Vec<String> = [String::from("https://www.imdb.com/title/tt3914054/?ref_=hm_stp_i_1_pvs_piv"), String::from("https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=10")].to_vec();

    // Create a future for each URL to fetch and parse the HTML
    // Using `futures::future::try_join_all` to run them concurrently
    // and collect the results into a vector of strings.
    // This will fetch the HTML content of each page and store it in `documents`.
    use futures::future::try_join_all;
    use reqwest::Error;
    let futures = urls.iter().map(|url| {
        let client = &client;
        async move {
            client.get(url).send().await?.text().await
        }
    });

    let documents: Vec<String> = try_join_all(futures).await?;
    let elapsed = start.elapsed();

    println!("Fetched and stored {} documents", documents.len());
    println!("Time taken to fetch and parse 100 pages: {:?}", elapsed);

    Ok(())
}
