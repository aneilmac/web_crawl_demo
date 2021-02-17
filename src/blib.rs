mod crawl_stream;

pub use crawl_stream::crawl_domain;
pub use crawl_stream::crawl_domain_with_client;
pub use crawl_stream::CrawlResult;

use reqwest::{Client, ClientBuilder, Result, Url};
use std::vec;



#[cfg(test)]
mod tests {
    use super::crawl_stream::tests;
    #[tokio::test]
    fn unique_url_list() {
      
    }
}
