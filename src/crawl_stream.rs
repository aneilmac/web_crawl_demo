use futures::{stream, future::FutureExt};
use reqwest::{Client, ClientBuilder, Url};
use select::document::Document;
use select::predicate::Name;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;
use stream::Stream;

pub struct CrawlResult {
    pub url: Url,
    pub html: reqwest::Result<Document>,
}

pub fn crawl_domain(url: Url) -> reqwest::Result<impl Stream<Item = CrawlResult>> {
  let builder = ClientBuilder::new();
  let client = builder.build()?;
  Ok(crawl_domain_with_client(client, url))
}

pub fn crawl_domain_with_client(client: Client, url: Url) -> impl Stream<Item = CrawlResult> {
  let stream_state = CrawlStreamState {
      client,
      visited: Default::default(),
      to_visit: {
          let mut v = BinaryHeap::<Url>::default();
          v.push(url);
          v
      },
  };

  stream::unfold(stream_state, move |stream_state| {
      let doc_tuple = pop_document(stream_state);
      doc_tuple.map(|d| {
          d.map(|(crawl_response, mut state)| {
              if let Ok(html) = &crawl_response.html {
                  let _ = state.push_document_links(html);
              }
              (crawl_response, state)
          })
      })
  })
}

struct CrawlStreamState {
    client: Client,
    visited: HashSet<Url>,
    to_visit: BinaryHeap<Url>,
}

impl CrawlStreamState {
    fn add_url_to_queue(&mut self, url: Url) -> Option<()> {
        if self.visited.contains(&url) {
            return None;
        }
        self.to_visit.push(url);
        Some(())
    }

    async fn document_for_url(&self, url: Url) -> CrawlResult {
        let res = self.client.get(url.clone()).send().await;
        match res {
            Err(e) => CrawlResult { url, html: Err(e) },
            Ok(t) => {
                let body = t.text().await;
                match body {
                    Err(e) => CrawlResult { url, html: Err(e) },
                    Ok(html) => CrawlResult {
                        url,
                        html: Ok(Document::from(html.as_str())),
                    },
                }
            }
        }
    }

    fn push_document_links(&mut self, html: &Document) -> std::io::Result<()> {
        let urls = html
            .find(Name("a")) // or Link
            .filter_map(|n| n.attr("href"))
            .filter_map(|raw_url| Url::from_str(raw_url).ok());

        for url in urls {
            let _ = self.add_url_to_queue(url);
        }
        Ok(())
    }
}

async fn pop_document(
    mut crawl_state: CrawlStreamState,
) -> Option<(CrawlResult, CrawlStreamState)> {
    loop {
        // Grab some arbitrary URL from the queue.
        let p = crawl_state.to_visit.pop();
        match p {
            None => {
                return None;
            }
            Some(url) => {
                // If the URL has been visited before we just loop round to
                // the next URL. Otherwise we index and grab the document.
                let is_unvisited = crawl_state.visited.insert(url.clone());
                if is_unvisited {
                    let doc_for_url = crawl_state.document_for_url(url).await;
                    return Some((doc_for_url, crawl_state));
                }
            }
        }
    }
}
