use futures::stream::StreamExt;
use reqwest::{Result, Url};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
pub async fn main() -> Result<()> {
    let url = Url::parse("https://www.linuxmint.com/").unwrap();
    println!("Crawling for {}:", &url);

    let mut stream = Box::pin(web_crawler_lib::crawl_domain(url)?);

    while let Some(value) = stream.next().await {
        println!("Got {}", value.url);
    }
    Ok(())
}
