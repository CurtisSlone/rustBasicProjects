use reqwest;
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]

async fn main() -> Result<(),Box<dyn Error>> {
    let url = "https://www.magic.agency";

    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);

    let selector = Selector::parse("h1").unwrap();

    for element in document.select(&selector) {
        println!("{}", element.text().collect::<Vec<_>>().concat());
    }

    Ok(())
    
}
