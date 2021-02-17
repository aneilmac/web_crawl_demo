mod crawl_stream;

use reqwest::Url;

use futures::stream::StreamExt;
pub use crawl_stream::crawl_domain;
pub use crawl_stream::crawl_domain_with_client;
pub use crawl_stream::CrawlResult;

pub async fn unique_url_list(url: Url) -> std::vec::Vec<Url> {
    let mut v = std::vec::Vec::new();
    let mut stream = Box::pin(crawl_domain(url));
    while let Some(result) = stream.next().await {
        v.push(result.url)
    }
    v
}

pub async fn unique_url_count(url: Url) -> usize {
    unique_url_list(url).await.iter().count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::unique_url_count(Url::from_str("https://beano.com")).
    }
}