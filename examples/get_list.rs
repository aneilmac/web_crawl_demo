use reqwest::{Result, Url};
use web_crawler_demo;

#[tokio::main]
pub async fn main() -> Result<()> {
    let url = Url::parse("https://www.reddit.com").unwrap();
    println!("Crawling for {}:", url);

    let list = web_crawler_demo::unique_url_list(url.clone()).await?;
    for url in list {
        println!("Url found: {}", url);
    }
    Ok(())
}
