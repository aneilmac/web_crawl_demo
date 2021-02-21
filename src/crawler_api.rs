//! This module represents the API functionality required to perform the
//! start-crawl, get-urls, and get-url-count calls.
mod add_domain;
mod domains;
mod get_urls;

pub use add_domain::add_domain;
pub use domains::Domains;
pub use get_urls::{get_domain_url_count, get_domain_urls, UrlListCountResult, UrlListResult};
