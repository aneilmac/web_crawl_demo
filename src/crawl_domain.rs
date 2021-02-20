use core::convert::TryFrom;
use serde::Deserialize;
use std::convert::AsRef;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use url::Url;
use warp::http::Response;

#[derive(Deserialize, Eq, Clone)]
#[serde(try_from = "&str")]
pub struct CrawlDomain {
    url: Url,
}

impl CrawlDomain {
    fn domain(&self) -> &str {
        // We can only be constructed if a domain exists, so can unwrap without
        // checking here.
        self.url.domain().unwrap()
    }
}

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

fn attempt_add_scheme(s: &str) -> Result<Url, url::ParseError> {
    let url = Url::parse(s);
    match url {
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            if !s.starts_with("https://") && !s.starts_with("http:") {
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
        println!("Building Domain from {}", s);
        println!("URL is from {:?}", Url::parse(s));
        let url = attempt_add_scheme(s)?;
        println!("URL is from {:?}", url);
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

impl warp::Reply for CrawlDomain {
    fn into_response(self) -> warp::reply::Response {
        Response::new(self.domain().to_owned().into())
    }
}
