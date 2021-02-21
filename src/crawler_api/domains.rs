use crate::crawl_domain::CrawlDomain;
use std::sync::{Arc, RwLock};

/// Collection of URLS for a given domain as known by the server.
#[derive(Debug, Default)]
pub struct DomainUrls {
    /// Is the server still crawling the domain for URLS or not?
    pub crawl_completed: bool,
    /// Collection of URLs so far.
    pub urls: std::vec::Vec<url::Url>,
}

pub type Urls = Arc<RwLock<DomainUrls>>;

type DomainCollection = std::collections::HashMap<CrawlDomain, Urls>;

/// Collection of all our domain keys mapped to a collection of known unique
/// URLs in the domain.
pub type Domains = Arc<RwLock<DomainCollection>>;
