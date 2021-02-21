use crate::crawl_domain::CrawlDomain;
use crate::crawler_api::domains::{DomainUrls, Domains};
use serde::Serialize;
use warp::Rejection;

/// Error case for when a Domain is was not found that was requested.
#[derive(Debug)]
struct DomainNotFound;

impl warp::reject::Reject for DomainNotFound {}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct UrlListResult {
    pub crawl_completed: bool,
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

/// Returns the complete list of URLs for a given domain as a json object. The
/// JSON object takes the form of:
///
/// ```text
/// {
///   "crawl_completed": [true|false]
///   "urls": ["https://foo.com", ...]
/// }
/// ```
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

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct UrlListCountResult {
    pub crawl_completed: bool,
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

/// Returns the complete count of URLs for a given domain as a json object. The
/// JSON object takes the form of:
///
/// ```text
/// {
///   "crawl_completed": [true|false]
///   "url_count": N
/// }
/// ```
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
