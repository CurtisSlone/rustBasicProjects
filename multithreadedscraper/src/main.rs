use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // List of URLs to scrape
    let urls = vec![
        "https://example.com",
        "https://example.org",
        "https://example.net",
    ];

    // Create an HTTP client
    let client = Client::new();

    // Create a vector to hold the tasks
    let mut tasks = Vec::new();

    // Loop through each URL and spawn a task for it
    for url in urls {
        let client = client.clone();
        let url = url.to_string();
        tasks.push(task::spawn(async move {
            if let Err(e) = scrape_url(&client, &url).await {
                eprintln!("Error scraping {}: {}", url, e);
            }
        }));
    }

    // Await all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }

    Ok(())
}

async fn scrape_url(client: &Client, url: &str) -> Result<(), Box<dyn Error>> {
    // Send HTTP GET request
    let response = client.get(url).send().await?.text().await?;

    // Parse the HTML content
    let document = Document::from(response.as_str()); // Convert String to &str

    // Example: Print all links
    for node in document.find(Name("a")).filter_map(|n| n.attr("href")) {
        println!("Found link: {}", node);
    }

    Ok(())
}
