use crate::crawl_domain::CrawlDomain;
use crate::crawler_api::domains::{Domains, Urls};
use futures::stream::StreamExt;
use reqwest::Client;
use warp::Rejection;
use web_crawler_lib::crawl_domain_with_client;

/// Error case for when a Domain is posted that already exists in our collection.
#[derive(Debug)]
struct DomainAlreadyAdded;

impl warp::reject::Reject for DomainAlreadyAdded {}

/// Error case for when a Domain does not have a http or https scheme.
#[derive(Debug)]
struct SchemeNotSupported;

impl warp::reject::Reject for SchemeNotSupported {}

/// Adds the given domain key to our list of domains and starts off the URL
/// crawl job on our new key.
pub async fn add_domain(
    domains: Domains,
    domain_key: CrawlDomain,
    client: Client,
) -> Result<CrawlDomain, Rejection> {
    // Only support crawls on http and https domains.
    if domain_key.as_ref().scheme() != "http" && domain_key.as_ref().scheme() != "https" {
        return Err(warp::reject::custom(SchemeNotSupported));
    }

    let mut domains = domains.write().unwrap();
    if domains.contains_key(&domain_key) {
        Err(warp::reject::custom(DomainAlreadyAdded))
    } else {
        let urls = Urls::default();
        let _ = domains.insert(domain_key.clone(), urls.clone());
        drop(domains);

        let url = domain_key.as_ref().clone();
        tokio::spawn(async move {
            // Note when we have completed the URL crawl.
            let _crawl_guard = CrawlCompleted { urls: urls.clone() };

            // If we can't connect to the address then early-exit
            if client.head(url.clone()).send().await.is_err() {
                return;
            }

            // Iterate through the stream, adding the URL to our domain's list.
            crawl_domain_with_client(client, url)
                .for_each_concurrent(None, move |crawl_result| {
                    urls.write().unwrap().urls.push(crawl_result.url);
                    futures::future::ready(())
                })
                .await;
        });
        Ok(domain_key)
    }
}

/// Helper RAII that triggers that `crawl_completed` is true on the given `Urls`
/// when this struct is destructed.
struct CrawlCompleted {
    /// URLs to update on destruction.
    urls: Urls,
}

impl Drop for CrawlCompleted {
    fn drop(&mut self) {
        self.urls.write().unwrap().crawl_completed = true;
    }
}
