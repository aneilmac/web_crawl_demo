use core::convert::TryFrom;
use serde::Deserialize;
use std::convert::AsRef;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use url::Url;
use warp::http::Response;

/// Struct that represents a URL's domain. This is internally backed by a URL
/// so we can retrieve the URL the Domain was constructed with, but in terms of
/// Equality and Hash tests only compare to the URL domain.
#[derive(Deserialize, Eq, Clone)]
#[serde(try_from = "&str")]
pub struct CrawlDomain {
    url: Url,
}

impl CrawlDomain {
    /// Returns the domain as a string/
    fn domain(&self) -> &str {
        // We can only be constructed if a domain exists, so can unwrap without
        // checking here.
        self.url.domain().unwrap()
    }
}

impl warp::Reply for CrawlDomain {
    fn into_response(self) -> warp::reply::Response {
        Response::new(self.domain().to_owned().into())
    }
}

// Convert the `CrawlDomain` back into the `Url` used to constuct it.
impl AsRef<Url> for CrawlDomain {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl std::fmt::Debug for CrawlDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrawlDomain")
            .field("domain", &self.domain())
            .field("url", &self.url)
            .finish()
    }
}

impl PartialEq for CrawlDomain {
    fn eq(&self, other: &CrawlDomain) -> bool {
        self.domain() == other.domain()
    }
}

impl Hash for CrawlDomain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.domain().hash(state);
    }
}

/// Simple helper method that lets us assume a https scheme if no scheme is
/// provided. If the parse of the string failed, attempts a reparse
/// by prepending the string with `https://`.
fn attempt_add_scheme(s: &str) -> Result<Url, url::ParseError> {
    let url = Url::parse(s);
    match url {
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            if !s.starts_with("https://") && !s.starts_with("http://") {
                Url::parse(&format!("https://{}", s))
            } else {
                url
            }
        }
        _ => url,
    }
}

impl FromStr for CrawlDomain {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = attempt_add_scheme(s)?;
        let _ = url
            .domain()
            .ok_or(url::ParseError::RelativeUrlWithoutBase)?;
        Ok(CrawlDomain { url })
    }
}

impl<'a> TryFrom<&'a str> for CrawlDomain {
    type Error = url::ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CrawlDomain::from_str(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_good_url() {
        let url = Url::parse("https://www.enhance.com").unwrap();
        let crawl_domain = CrawlDomain::from_str(url.as_str()).unwrap();
        assert_eq!(url, crawl_domain.url)
    }

    #[test]
    fn test_missing_scheme() {
        let url = "www.enhance.com/hello/world.html";
        let crawl_domain = CrawlDomain::from_str(url).unwrap();
        assert_eq!(crawl_domain.domain(), "www.enhance.com");
    }

    #[test]
    fn test_no_domain() {
        let url = "https://127.0.0.1/";
        let crawl_domain = CrawlDomain::from_str(url);
        assert!(crawl_domain.is_err());
    }
}
