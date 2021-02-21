use crate::crawl_domain::CrawlDomain;
use futures::stream::StreamExt;
use reqwest::Client;
use serde::Serialize;
use std::boxed::Box;
use std::sync::{Arc, RwLock};
use warp::reply::Json;
use warp::Rejection;
use web_crawler_lib::crawl_domain_with_client;

/// Collection of URLS for a given domain as known by the server.
#[derive(Serialize, Debug, Default)]
pub struct DomainUrls {
    /// Is the server still crawling the domain for URLS or not?
    crawl_completed: bool,
    /// Collection of URLs so far.
    urls: std::vec::Vec<url::Url>,
}

type Urls = Arc<RwLock<DomainUrls>>;
type DomainCollection = std::collections::HashMap<CrawlDomain, Urls>;

/// Collection of all our domain keys mapped to a collection of known unique
/// URLs in the domain.
pub type Domains = Arc<RwLock<DomainCollection>>;

#[derive(Debug)]
struct DomainInaccessible;

impl warp::reject::Reject for DomainInaccessible {}

pub async fn is_accessible(client: Client, url: url::Url) -> Result<(), Rejection> {
    match client.head(url).send().await.is_ok() {
        true => Ok(()),
        false => Err(warp::reject::custom(DomainInaccessible)),
    }
}

#[derive(Debug)]
struct DomainAlreadyAdded;

impl warp::reject::Reject for DomainAlreadyAdded {}

pub async fn add_domain(
    domains: Domains,
    domain_key: CrawlDomain,
    client: Client,
) -> Result<CrawlDomain, Rejection> {
    let mut domains = domains.write().unwrap();
    if domains.contains_key(&domain_key) {
        Err(warp::reject::custom(DomainAlreadyAdded))
    } else {
        let urls = Urls::default();
        let _ = domains.insert(domain_key.clone(), urls.clone());
        drop(domains);

        let url = domain_key.as_ref().clone();
        tokio::spawn(async move {
            let mut crawl_stream = Box::pin(crawl_domain_with_client(client, url));
            while let Some(crawl_result) = crawl_stream.next().await {
                urls.write().unwrap().urls.push(crawl_result.url);
            }
            // Finished trawling URLs, not that we are now completed.
            urls.write().unwrap().crawl_completed = true;
        });
        Ok(domain_key)
    }
}

pub async fn get_domain_urls(domains: Domains, domain_key: CrawlDomain) -> Result<Json, Rejection> {
    let domains = domains.read().unwrap();
    if let Some(urls) = domains.get(&domain_key) {
        let urls = urls.read().unwrap();
        return Ok(warp::reply::json(&*urls));
    }
    Err(warp::reject::not_found())
}

#[derive(Serialize, Debug)]
pub struct UrlListCountResult {
    crawl_completed: bool,
    url_count: usize,
}

impl UrlListCountResult {
    pub fn create(domain_urls: &DomainUrls) -> Self {
        Self {
            crawl_completed: domain_urls.crawl_completed,
            url_count: domain_urls.urls.iter().count(),
        }
    }
}

pub async fn get_domain_url_count(
    domains: Domains,
    domain_key: CrawlDomain,
) -> Result<Json, Rejection> {
    let domains = domains.read().unwrap();
    if let Some(urls) = domains.get(&domain_key) {
        let urls = urls.read().unwrap();
        return Ok(warp::reply::json(&UrlListCountResult::create(&urls)));
    }
    Err(warp::reject::not_found())
}
