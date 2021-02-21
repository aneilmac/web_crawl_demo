use crate::crawler_api::domains::{Domains, DomainUrls};
use crate::crawl_domain::CrawlDomain;
use serde::Serialize;
use warp::reply::Json;
use warp::Rejection;

/// Error case for when a Domain is was not found that was requested.
#[derive(Debug)]
struct DomainNotFound;

impl warp::reject::Reject for DomainNotFound {}

/// Returns the complete list of URLs for a given domain as a json object. The 
/// JSON object takes the form of:
/// 
/// ```text
/// {
///   "crawl_completed": [true|false]
///   "urls": ["https://foo.com", ...]
/// }
/// ```
pub async fn get_domain_urls(domains: Domains, domain_key: CrawlDomain) -> Result<Json, Rejection> {
  let domains = domains.read().unwrap();
  if let Some(urls) = domains.get(&domain_key) {
      let urls = urls.read().unwrap();
      return Ok(warp::reply::json(&*urls));
  }
  Err(warp::reject::custom(DomainNotFound))
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
) -> Result<Json, Rejection> {
  let domains = domains.read().unwrap();
  if let Some(urls) = domains.get(&domain_key) {
      let urls = urls.read().unwrap();
      return Ok(warp::reply::json(&UrlListCountResult::create(&urls)));
  }
  Err(warp::reject::custom(DomainNotFound))
}
