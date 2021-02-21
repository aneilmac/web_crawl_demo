mod crawl_domain;
mod crawler_api;
mod tests;

use crawl_domain::CrawlDomain;
use crawler_api::*;
use reqwest::Client;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::Filter;

#[tokio::main]
async fn main() {
    let ip = get_command_line_socket_addr();
    println!("Started crawler server at {}", ip);

    let crawler = build_crawler_domains();
    let crawler_routes = build_post_domain(crawler.clone())
        .or(build_get_urls(crawler.clone()))
        .or(build_get_urls_count(crawler));

    warp::serve(crawler_routes).run(ip).await;
}

/// Creates our base `<HOST>/crawler/domains/` for all filters.
fn build_crawler_domains() -> impl Filter<Extract = (Domains,), Error = warp::Rejection> + Clone {
    let domains = Domains::default();
    let domains = warp::any().map(move || domains.clone());
    warp::path!("crawler" / "domains" / ..).and(domains)
}

/// Creates our filter system for POSTing the  `Url` at:
/// `<HOST>/crawler/domains/`
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

/// Creates our filter system for grabbing the `UrlListResult` from GET:
/// `<HOST>/crawler/domains/<DOMAIN>/urls`
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

/// Creates our filter system for grabbing the `UrlListCountResult` from GET:
/// `<HOST>/crawler/domains/<DOMAIN>/urls/count`
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

/// Grab the IP address given by the command line,
/// or default to `127.0.0.1:8080` if not provided or incorrectly formatted.
fn get_command_line_socket_addr() -> SocketAddr {
    let args: Vec<String> = env::args().collect();
    let mut iter = args.iter();
    let _ = iter.next(); // Ignore app name.
    iter.next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            8080,
        ))
}
