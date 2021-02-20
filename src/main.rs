mod crawl_domain;
mod tests;

use warp::reply::Json;
use warp::{Filter, Rejection};

use crawl_domain::CrawlDomain;
use futures::stream::StreamExt;
use reqwest::Client;
use serde::Serialize;
use std::boxed::Box;
use std::sync::{Arc, Mutex};
use web_crawler_lib::crawl_domain_with_client;

#[derive(Default, Serialize, Debug)]
struct UrlListResult {
    crawl_completed: bool,
    urls: std::vec::Vec<String>,
}

#[derive(Serialize, Debug)]
struct UrlListCountResult {
    crawl_completed: bool,
    url_count: usize,
}

impl UrlListCountResult {
    fn create(result: &UrlListResult) -> UrlListCountResult {
        UrlListCountResult {
            crawl_completed: result.crawl_completed,
            url_count: result.urls.iter().count(),
        }
    }
}

type DomainCollection = std::collections::HashMap<CrawlDomain, UrlListResult>;
type Domains = Arc<Mutex<DomainCollection>>;

#[derive(Debug)]
struct DomainAlreadyAdded;

impl warp::reject::Reject for DomainAlreadyAdded {}

#[tokio::main]
async fn main() {
    let domains = Domains::default();
    let domains = warp::any().map(move || domains.clone());

    let crawler = warp::path!("crawler" / "domains" / ..).and(domains);

    let client = Client::new();
    let client = warp::any().map(move || client.clone());

    let crawler_routes = crawler
        .clone()
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(client)
        .and_then(add_domain)
        .or(crawler
            .clone()
            .and(warp::get())
            .and(warp::path::param::<CrawlDomain>())
            .and(warp::path("urls"))
            .and(warp::path::end())
            .and_then(get_domain_urls))
        .or(crawler
            .and(warp::get())
            .and(warp::path::param::<CrawlDomain>())
            .and(warp::path("urls"))
            .and(warp::path("count"))
            .and(warp::path::end())
            .and_then(get_domain_url_count));

    warp::serve(crawler_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn get_domain_urls(domains: Domains, domain: CrawlDomain) -> Result<Json, Rejection> {
    let domains = domains.lock().unwrap();
    if let Some((_, result)) = domains.get_key_value(&domain) {
        return Ok(warp::reply::json(result));
    }
    Err(warp::reject::not_found())
}

async fn get_domain_url_count(domains: Domains, domain: CrawlDomain) -> Result<Json, Rejection> {
    let domains = domains.lock().unwrap();
    if let Some((_, result)) = domains.get_key_value(&domain) {
        return Ok(warp::reply::json(&UrlListCountResult::create(result)));
    }
    Err(warp::reject::not_found())
}

async fn add_domain(
    domains_arc: Domains,
    domain: CrawlDomain,
    client: Client,
) -> Result<CrawlDomain, Rejection> {
    let mut domains = domains_arc.lock().unwrap();

    if domains.contains_key(&domain) {
        Err(warp::reject::custom(DomainAlreadyAdded))
    } else {
        let _ = domains.insert(domain.clone(), UrlListResult::default());
        drop(domains);

        let out_domain = domain.clone();
        tokio::spawn(async move {
            let mut crawl_stream =
                Box::pin(crawl_domain_with_client(client, domain.as_ref().clone()));
            while let Some(crawl_result) = crawl_stream.next().await {
                let mut domains = domains_arc.lock().unwrap();
                if let Some(result) = domains.get_mut(&domain) {
                    result.urls.push(crawl_result.url.into_string());
                }
            }

            // Finished trawling URLs, not that we are now completed.
            let mut domains = domains_arc.lock().unwrap();
            if let Some(result) = domains.get_mut(&domain) {
                result.crawl_completed = true;
            }
        });
        Ok(out_domain)
    }
}
