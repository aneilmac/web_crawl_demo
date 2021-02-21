//! Webserver unit tests exist here.
#![cfg(test)]

use super::*;

/// Tests posting a simple URL.
#[tokio::test]
async fn test_post() {
    let crawler = build_crawler_domains();
    let post_domain = build_post_domain(crawler);

    let url = "simple.test";

    assert_eq!(
        warp::test::request()
            .method("POST")
            .path("/crawler/domains")
            .json(&url)
            .filter(&post_domain)
            .await
            .unwrap()
            .domain(),
        url
    )
}

/// Tests posting a URL with an invalid scheme.
#[tokio::test]
async fn test_invalid_scheme_post() {
    let crawler = build_crawler_domains();
    let post_domain = build_post_domain(crawler);

    let url = "ftp://simple.test";

    assert!(warp::test::request()
        .method("POST")
        .path("/crawler/domains")
        .json(&url)
        .filter(&post_domain)
        .await
        .is_err())
}

/// Tests posting the same URL twice.
#[tokio::test]
async fn test_duplicate_post() {
    let crawler = build_crawler_domains();
    let post_domain = build_post_domain(crawler);

    let build_post_test = |url| {
        warp::test::request()
            .method("POST")
            .path("/crawler/domains")
            .json(&url)
            .filter(&post_domain)
    };

    assert!(build_post_test("http://simple.test").await.is_ok());
    assert!(build_post_test("https://simple.test").await.is_err());
}

/// Tests getting an arbitrary result from a posted URL.
#[tokio::test]
async fn test_get_urls_good_domain() {
    let crawler = build_crawler_domains();
    let post_domain = build_post_domain(crawler.clone());
    let get_urls_domain = build_get_urls(crawler);

    let url = "some.test";

    let crawl_domain = warp::test::request()
        .method("POST")
        .path("/crawler/domains")
        .json(&url)
        .filter(&post_domain)
        .await
        .unwrap();

    assert!(warp::test::request()
        .path(&format!("/crawler/domains/{}/urls", crawl_domain.domain()))
        .filter(&get_urls_domain)
        .await
        .is_ok())
}

/// Tests getting an arbitrary result from a non-posted URL.
#[tokio::test]
async fn test_get_urls_bad_domain() {
    let crawler = build_crawler_domains();
    let get_urls_domain = build_get_urls(crawler);

    assert!(warp::test::request()
        .path("/crawler/domains/www.enhance.com/urls")
        .filter(&get_urls_domain)
        .await
        .is_err())
}

/// Tests getting an arbitrary result from a posted URL.
#[tokio::test]
async fn test_get_urls_count_good_domain() {
    let crawler = build_crawler_domains();
    let post_domain = build_post_domain(crawler.clone());
    let get_urls_count_domain = build_get_urls_count(crawler);

    let url = "some.test";

    let crawl_domain = warp::test::request()
        .method("POST")
        .path("/crawler/domains")
        .json(&url)
        .filter(&post_domain)
        .await
        .unwrap();

    assert!(warp::test::request()
        .path(&format!(
            "/crawler/domains/{}/urls/count",
            crawl_domain.domain()
        ))
        .filter(&get_urls_count_domain)
        .await
        .is_ok())
}

/// Tests getting an arbitrary result from a non-posted URL.
#[tokio::test]
async fn test_get_urls_count_bad_domain() {
    let crawler = build_crawler_domains();
    let get_urls_count_domain = build_get_urls_count(crawler);

    assert!(warp::test::request()
        .path("/crawler/domains/www.enhance.com/urls/count")
        .filter(&get_urls_count_domain)
        .await
        .is_err())
}

/// Tests getting a result from a posted URL that is inaccessible to `reqwest`.
#[tokio::test]
async fn test_get_urls_no_access() {
    let crawler = build_crawler_domains();
    let post_domain = build_post_domain(crawler.clone());
    let get_urls_count_domain = build_get_urls_count(crawler);

    let url = "some.invalid";

    let crawl_domain = warp::test::request()
        .method("POST")
        .path("/crawler/domains")
        .json(&url)
        .filter(&post_domain)
        .await
        .unwrap();

    assert_eq!(
        warp::test::request()
            .path(&format!(
                "/crawler/domains/{}/urls/count",
                crawl_domain.domain()
            ))
            .filter(&get_urls_count_domain)
            .await
            .unwrap()
            .url_count,
        0
    );
}
