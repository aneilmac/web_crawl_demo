use web_crawler_demo;
use reqwest::{Result, Url};
use futures::stream::StreamExt;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
pub async fn main() -> Result<()> {
    let url = Url::parse("https://www.linuxmint.com/").unwrap();
    println!("Crawling for {}:", &url);
    
    let mut stream = Box::pin(web_crawler_demo::crawl_domain(url)?);

    while let Some(value) = stream.next().await {
      println!("Got {}", value.url);
  }
  Ok(())
}
