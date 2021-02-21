mod crawl_domain;
mod crawler_api;
mod tests;

use crawl_domain::CrawlDomain;
use crawler_api::{
    add_domain, get_domain_url_count, get_domain_urls, Domains, UrlListCountResult, UrlListResult,
};
use reqwest::Client;
use warp::Filter;

#[tokio::main]
async fn main() {
    let crawler = build_crawler_domains();

    let crawler_routes = build_post_domain(crawler.clone())
        .or(build_get_urls(crawler.clone()))
        .or(build_get_urls_count(crawler));

    warp::serve(crawler_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn build_crawler_domains() -> impl Filter<Extract = (Domains,), Error = warp::Rejection> + Clone {
    let domains = Domains::default();
    let domains = warp::any().map(move || domains.clone());
    warp::path!("crawler" / "domains" / ..).and(domains)
}

fn build_post_domain(
    crawler: impl Filter<Extract = (Domains,), Error = warp::Rejection> + Clone,
) -> impl Filter<Extract = (CrawlDomain,), Error = warp::Rejection> + Clone {
    let client = Client::new();
    let client = warp::any().map(move || client.clone());
    crawler
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(client)
        .and_then(add_domain)
}

fn build_get_urls(
    crawler: impl Filter<Extract = (Domains,), Error = warp::Rejection> + Clone,
) -> impl Filter<Extract = (UrlListResult,), Error = warp::Rejection> + Clone {
    crawler
        .and(warp::get())
        .and(warp::path::param::<CrawlDomain>())
        .and(warp::path("urls"))
        .and(warp::path::end())
        .and_then(get_domain_urls)
}

fn build_get_urls_count(
    crawler: impl Filter<Extract = (Domains,), Error = warp::Rejection> + Clone,
) -> impl Filter<Extract = (UrlListCountResult,), Error = warp::Rejection> + Clone {
    crawler
        .and(warp::get())
        .and(warp::path::param::<CrawlDomain>())
        .and(warp::path("urls"))
        .and(warp::path("count"))
        .and(warp::path::end())
        .and_then(get_domain_url_count)
}
