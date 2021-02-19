mod tests;

use futures::stream;
use reqwest::{Client, ClientBuilder, Result, Url};
use select::document::Document;
use select::predicate::{Name, Predicate};
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;
use stream::Stream;
use std::vec;
use futures::stream::StreamExt;

/// Returns a Stream that runs over all URLs in the given domain of `url`.
///
/// This is the preferred method to crawl over URL, as the stream can yield 
/// control back to the caller as it goes.
/// 
/// For multiple requests it is recommended you use the same client across 
/// requests. See `crawl_domain_with_client`.
/// 
/// ## Example
/// 
/// ```rust
/// use web_crawler_demo;
/// use reqwest::{Result, Url};
/// use futures::stream::StreamExt;
/// 
/// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
/// pub async fn main() -> Result<()> {
///     let url = Url::parse("https://www.linuxmint.com/").unwrap();
///     println!("Crawling for {}:", &url);
///
///     let mut stream = Box::pin(web_crawler_demo::crawl_domain(url)?);
/// 
///     while let Some(value) = stream.next().await {
///         println!("Got {}", value.url);
///     }
///     Ok(())
/// }
/// ```
/// 
/// Would give live output like:
/// 
/// ```text
/// Crawling for https://www.linuxmint.com/:
/// Got https://www.linuxmint.com/
/// Got https://www.linuxmint.com/topstories/topstories.css
/// Got https://www.linuxmint.com/teams.php
/// Got https://www.linuxmint.com/teams.php#content
/// Got https://www.linuxmint.com/store.php
/// Got https://www.linuxmint.com/store_tshirts.php
/// Got https://www.linuxmint.com/store_tshirts.php#content
/// Got https://www.linuxmint.com/store_mintbox3.php
/// Got https://www.linuxmint.com/store_mintbox3.php#content
/// Got https://www.linuxmint.com/store_mintbox.php
/// Got https://www.linuxmint.com/store_mintbox.php#content
/// Got https://www.linuxmint.com/store_computers.php
/// Got https://www.linuxmint.com/store_computers.php#content
/// Got https://www.linuxmint.com/store.php#content
/// Got https://www.linuxmint.com/sponsors.php
/// Got https://www.linuxmint.com/sponsors_info.php
/// Got https://www.linuxmint.com/sponsors_info.php#content
/// Got https://www.linuxmint.com/sponsors.php#content
/// Got https://www.linuxmint.com/screenshots.php
/// ...
/// ```
pub fn crawl_domain(url: Url) -> Result<impl Stream<Item = CrawlResult>> {
    let builder = ClientBuilder::new();
    let client = builder.build()?;
    Ok(crawl_domain_with_client(client, url))
}

/// Alternative to `crawl_domain` that accepts a `client` object.
/// 
/// In cases where multiple requests are made, reuse of the same client is 
/// better than creating a new `Client` object for each call.
pub fn crawl_domain_with_client(client: Client, url: Url) -> impl Stream<Item = CrawlResult> {
    let init_state = CrawlStreamState::create(client, url);
    // From our initial state attempt to generate a stream.
    stream::unfold(init_state, |state| state.pop_document())
}

/// Returns a complete list of all URLs visited in the given domain of `url`.
///  
/// This task does not complete until all URLs are visited and as such may not 
/// be suitable for large domains. See `crawl_domain` for the `Stream` 
/// equivalent to this `Future`.
/// 
/// For multiple requests it is recommended you use the same client across 
/// requests. See `unique_url_list_with_client`.
/// 
/// ## Example
/// 
/// ```rust
/// use web_crawler_demo;
/// use reqwest::{Result, Url};
/// 
/// #[tokio::main]
/// pub async fn main() -> Result<()> {
///     let url = Url::parse("https://www.enhance.com/").unwrap();
///     println!("Crawling for {}:", &url);
///     
///     let list = web_crawler_demo::unique_url_list(url).await?;
///     for u in list {
///       println!("Url found: {}", u);
///     }
///     Ok(())
/// }
/// ```
/// 
/// Would give output:
/// 
/// ```text
/// Crawling for https://www.enhance.com/:
/// Url found: https://www.enhance.com/
/// Url found: https://www.enhance.com/styles.208feb938cace1c3135d.css
/// Url found: https://www.enhance.com/favicon.ico
/// ```
pub async fn unique_url_list(url: Url) -> Result<vec::Vec<Url>> {
    let builder = ClientBuilder::new();
    let client = builder.build()?;
    Ok(unique_url_list_with_client(client, url).await)
}

/// Alternative to `unique_url_list` that accepts a `client` object.
/// 
/// In cases where multiple requests are made, reuse of the same client is 
/// better than creating a new `Client` object for each call.
pub async fn unique_url_list_with_client(client: Client, url: Url) -> vec::Vec<Url> {
    crawl_domain_with_client(client, url)
        .map(|r| r.url)
        .collect()
        .await
}

/// Returns a complete count of all URLs visited in the given domain of `url`.
///  
/// This task does not complete until all URLs are visited and as such may not 
/// be suitable for large domains. See `crawl_domain` for the `Stream` 
/// equivalent to this `Future`.
/// 
/// For multiple requests it is recommended you use the same client across 
/// requests. See `unique_url_count_with_client`.
/// 
/// ## Example
/// 
/// ```rust
/// use web_crawler_demo;
/// use reqwest::{Result, Url};
/// 
/// #[tokio::main]
/// pub async fn main() -> Result<()> {
///     let url = Url::parse("https://www.enhance.com/").unwrap();
///     println!("Crawling for {}:", &url);
///     
///     let count = web_crawler_demo::unique_url_count(url).await?;
///     println!("Urls found: {}", count);
///     Ok(())
/// }
/// ```
/// 
/// Would give output:
/// 
/// ```text
/// Crawling for https://www.enhance.com/:
/// Urls found: 3
/// ```
pub async fn unique_url_count(url: Url) -> Result<usize> {
    let builder = ClientBuilder::new();
    let client = builder.build()?;
    Ok(unique_url_count_with_client(client, url).await)
}

/// Alternative to `unique_url_count` that accepts a `client` object.
/// 
/// In cases where multiple requests are made, reuse of the same client is 
/// better than creating a new `Client` object for each call.
pub async fn unique_url_count_with_client(client: Client, url: Url) -> usize {
    crawl_domain_with_client(client, url)
        .fold(0, |i, _| std::future::ready(i + 1))
        .await
}

/// CrawlResult is output of a crawl.
pub struct CrawlResult {
    /// A given URL that was crawled.
    pub url: Url,
    /// The result of the crawl for the given URL, either an error if the crawl
    /// failed or a Document object representing the page.
    pub html: Result<Document>,
}

/// The current state of the CrawlStream.
struct CrawlStreamState {
    /// The client used to do crawl requests.
    client: Client,
    /// The collection of unique URLS that have already been processed.
    visited: HashSet<Url>,
    /// Collection of queued items to visit, may contain duplicates.
    to_visit: BinaryHeap<Url>,
}

impl CrawlStreamState {
    /// Initializes a `CrawlStreamState` with a `Client`, `client`, for HTTP
    /// requests, and a URL, `url`, to be the starting point for crawling a
    /// particular domain.
    fn create(client: Client, url: Url) -> Self {
        Self {
            client,
            visited: Default::default(),
            to_visit: {
                let mut v = BinaryHeap::<Url>::default();
                v.push(url);
                v
            },
        }
    }

    /// Adds th given URL, `url` to our list of URLs that are to be visited.
    ///
    /// This function returns `None` if `url` already exists in our collection
    /// of already visited URLs, otherwise `Some(())` is returned when the `url`
    /// is successfully added to the queue of URLs to visit. The same URL can be
    /// added successfully multiple times if it has not been visited at least
    /// once.
    fn add_url_to_queue(&mut self, url: Url) -> Option<()> {
        if self.visited.contains(&url) {
            return None;
        }
        self.to_visit.push(url);
        Some(())
    }

    /// Given a URL, url, attempts to retrieve the document for the given URL
    /// and returns the result in a `CrawlResult`.
    ///
    /// All documents are retrieved via the GET HTTP method.
    async fn document_for_url(&self, url: &Url) -> CrawlResult {
        CrawlResult {
            url: url.clone(),
            html: {
                let res = self.client.get(url.clone()).send().await;
                match res {
                    Err(e) => Err(e),
                    Ok(t) => {
                        let body = t.text().await;
                        match body {
                            Err(e) => Err(e),
                            Ok(html) => Ok(Document::from(html.as_str())),
                        }
                    }
                }
            },
        }
    }

    /// Given a URL, `url` and `Document`, `document`, goes through all valid
    /// href tags in the document, and if they have the same domain as `url`,
    /// append them to the to-visit queue when applicable.
    fn push_document_links(&mut self, document_url: &Url, html: &Document) {
        let urls = html
            .find(Name("a").or(Name("link")))
            .filter_map(|n| n.attr("href"))
            .filter_map(|raw_url| {
                Url::from_str(raw_url)
                    .or_else(|e| {
                        // If the URL is relative then `Url::parse` will fail. 
                        // We can try again by using the document URL
                        // as our base.
                        if e == url::ParseError::RelativeUrlWithoutBase {
                            document_url.join(raw_url)
                        } else {
                            Err(e)
                        }
                    })
                    .ok()
            })
            // Ensure URL is tied to our domain.
            .filter(|url| document_url.domain() == url.domain());

        // Take our URL collection and insert it into the queue.
        for url in urls {
            let _ = self.add_url_to_queue(url);
        }
    }

    /// Consumes the `CrawlState` and returns a tuple containing a
    /// `CrawlResult` for an arbitrary URL in the queue, and a new `CrawlState`.
    ///
    /// When there are no URLs to visit `None` is returned.
    ///
    /// The produced `CrawlState` marks the returned `CrawlResult` URL as
    /// visited, and has all applicable domain links from the given page added
    /// to the visit queue.
    async fn pop_document(mut self) -> Option<(CrawlResult, Self)> {
        loop {
            // Grab some arbitrary URL from the queue.
            let p = self.to_visit.pop();
            match p {
                None => {
                    // End the crawl for good. Our to-visit queue has been
                    // fully consumed.
                    return None;
                }
                Some(url) => {
                    // If the URL has been visited before we just loop round to
                    // the next URL. Otherwise we index and grab the document.
                    let is_unvisited = self.visited.insert(url.clone());
                    if is_unvisited {
                        let doc_for_url = self.document_for_url(&url).await;

                        // The newly produced document may contain links to
                        // additional URLs to index within this repo. Add these
                        // to our to-visit queue if applicable.
                        if let Ok(html) = &doc_for_url.html {
                            self.push_document_links(&url, html);
                        }
                        return Some((doc_for_url, self));
                    }
                }
            }
        }
    }
}
