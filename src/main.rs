mod tests;

use warp::{Filter, Rejection};
use url::Url;
use std::sync::{Arc, Mutex};

type UrlList = std::vec::Vec<Url>;
type DomainCollection = std::collections::HashMap<Url, UrlList>;

async fn add_domain(s: String) -> Result<&'static str, Rejection> {
    Ok("Hello world")
}

async fn get_domain_urls(s: String) -> Result<&'static str, Rejection> {
    Ok("urls")
}

async fn get_domain_url_count(s: String) -> Result<&'static str, Rejection> {
    Ok("count")
}

#[tokio::main]
async fn main() {
    let _domain_collection = Arc::new(Mutex::new(DomainCollection::default()));

    let crawler = warp::path!("crawler" / "domains" / ..);
    let crawler_routes = crawler
        .and(warp::post())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(add_domain)
        .or(crawler
            .and(warp::get())
            .and(warp::path::param::<String>())
            .and(warp::path("urls"))
            .and(warp::path::end())
            .and_then(get_domain_urls))
        .or(crawler
            .and(warp::get())
            .and(warp::path::param::<String>())
            .and(warp::path("urls"))
            .and(warp::path("count"))
            .and(warp::path::end())
            .and_then(get_domain_url_count));

    warp::serve(crawler_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}