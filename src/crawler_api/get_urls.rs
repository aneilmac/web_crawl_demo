use crate::crawl_domain::CrawlDomain;
use crate::crawler_api::domains::{DomainUrls, Domains};
use serde::Serialize;
use warp::Rejection;

/// Error case for when a Domain is was not found that was requested.
#[derive(Debug)]
struct DomainNotFound;

impl warp::reject::Reject for DomainNotFound {}

/// The result returned to the user when they request the list of all crawled
/// URLs associated with a given domain.
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct UrlListResult {
    /// Flag indicating if the server is still crawling the given domain.
    pub crawl_completed: bool,
    // All URLs collected for the given domain (so-far.)
    pub urls: std::vec::Vec<url::Url>,
}

impl UrlListResult {
    pub fn create(domain_urls: &DomainUrls) -> Self {
        Self {
            crawl_completed: domain_urls.crawl_completed,
            urls: domain_urls.urls.clone(),
        }
    }
}

impl warp::Reply for UrlListResult {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}

/// Returns the complete list of URLs for a given domain.
pub async fn get_domain_urls(
    domains: Domains,
    domain_key: CrawlDomain,
) -> Result<UrlListResult, Rejection> {
    let domains = domains.read().unwrap();
    if let Some(urls) = domains.get(&domain_key) {
        let urls = urls.read().unwrap();
        return Ok(UrlListResult::create(&urls));
    }
    Err(warp::reject::custom(DomainNotFound))
}

/// The result returned to the user when they request the list of all crawled
/// URLs associated with a given domain.
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct UrlListCountResult {
    /// Flag indicating if the server is still crawling the given domain.
    pub crawl_completed: bool,
    // Count of all URLs collected for the given domain (so-far.)
    pub url_count: usize,
}

impl UrlListCountResult {
    pub fn create(domain_urls: &DomainUrls) -> Self {
        Self {
            crawl_completed: domain_urls.crawl_completed,
            url_count: domain_urls.urls.iter().count(),
        }
    }
}

impl warp::Reply for UrlListCountResult {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}

/// Returns the complete count of URLs for a given domain.
pub async fn get_domain_url_count(
    domains: Domains,
    domain_key: CrawlDomain,
) -> Result<UrlListCountResult, Rejection> {
    let domains = domains.read().unwrap();
    if let Some(urls) = domains.get(&domain_key) {
        let urls = urls.read().unwrap();
        return Ok(UrlListCountResult::create(&urls));
    }
    Err(warp::reject::custom(DomainNotFound))
}
