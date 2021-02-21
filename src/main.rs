mod crawl_domain;
mod crawler_api;
mod tests;

use crawl_domain::CrawlDomain;
use crawler_api::{add_domain, get_domain_url_count, get_domain_urls, Domains};
use reqwest::Client;
use warp::Filter;

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
