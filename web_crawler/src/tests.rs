//! CrawlStream unit tests exist here.
#![cfg(test)]

use super::*;
use mockito::{mock, Mock};

/// Helper method to generate a simple `CrawlStreamState`.
fn default_state() -> CrawlStreamState {
    let client = Client::new();
    let url = Url::parse("https://www.enhance.com/").unwrap();
    CrawlStreamState::create(client, url)
}

/// Tests the default construction of State fro a URL.
#[test]
fn test_create_state() {
    let crawl_state = default_state();
    assert_eq!(crawl_state.visited.iter().count(), 0);
    assert_eq!(crawl_state.to_visit.iter().count(), 1);
}

/// Tests adding an unvisited URL to the to-visit list.
#[test]
fn test_add_unvisited() {
    let mut crawl_state = default_state();
    let url = Url::from_str("https://www.enhance.com/").unwrap();
    let res = crawl_state.add_url_to_queue(url);
    assert_eq!(res, Some(()));
    assert_eq!(crawl_state.visited.iter().count(), 0);
    assert_eq!(crawl_state.to_visit.iter().count(), 2);
}

/// Tests adding a visited URL to the to-visit list.
#[test]
fn test_add_visited() {
    let mut crawl_state = default_state();
    let url = Url::from_str("https://www.google.com/").unwrap();
    crawl_state.visited.insert(url.clone());
    let res = crawl_state.add_url_to_queue(url);
    assert_eq!(res, None);
    assert_eq!(crawl_state.visited.iter().count(), 1);
    assert_eq!(crawl_state.to_visit.iter().count(), 1);
}

/// Tests accessing a nonexistant document.
#[tokio::test]
async fn test_document_for_url_failure() {
    let crawl_state = default_state();
    let url = Url::parse("https://foo.invalid/failure.html").unwrap();
    let result = crawl_state.document_for_url(&url).await;
    assert!(result.is_err())
}

/// Attempts to create a valid URL to the `res` folder which contains
/// dummy indexable file: `simple.html`.
fn simple_html() -> Mock {
    mock("GET", "/simple.html")
        .with_status(201)
        .with_header("content-type", "text/plain")
        .with_header("x-api-key", "1234")
        .with_body(include_str!("../res/simple.html"))
        .create()
}

/// Create a URL to the resource `filename` in the mockio mock server.
fn mock_url(filename: &str) -> Url {
    let url = Url::parse(&mockito::server_url()).unwrap();
    url.join(filename).unwrap()
}

/// Tests accessing a document and testing it was retrieved correctly.
#[tokio::test]
async fn test_document_for_url_success() {
    let _m = simple_html();
    let crawl_state = default_state();
    let result = crawl_state.document_for_url(&mock_url("simple.html")).await;
    assert_eq!(
        result.unwrap(),
        Html::parse_document(include_str!("../res/simple.html"))
    )
}

/// Tests processing a document in the `to_visit` list, which triggers the
/// crawl state to change.
#[tokio::test]
async fn test_pop() {
    let _m = simple_html();

    let client = Client::new();
    let url = mock_url("simple.html");
    let crawl_state = CrawlStreamState::create(client, url.clone());

    if let Some((result, new_state)) = crawl_state.pop_document().await {
        assert_eq!(new_state.visited.iter().count(), 1);
        assert_eq!(new_state.to_visit.iter().count(), 1);

        assert_eq!(result.url, url);
        assert!(new_state.visited.contains(&url));
        assert_eq!(
            new_state.to_visit.peek().unwrap(),
            &mock_url("link_node.html")
        );
    } else {
        panic!("Expected valid document pop.")
    }
}

/// Attempts to create a valid URL to the `res` folder which contains
/// dummy indexable file: `self_ref.html`.
fn self_ref_html() -> Mock {
    mock("GET", "/self_ref.html")
        .with_status(201)
        .with_header("content-type", "text/plain")
        .with_header("x-api-key", "1234")
        .with_body(include_str!("../res/self_ref.html"))
        .create()
}

/// Attempts to create a valid URL to the `res` folder which contains
/// dummy indexable file: `link_node.html`.
fn link_node_html() -> Mock {
    mock("GET", "/link_node.html")
        .with_status(201)
        .with_header("content-type", "text/plain")
        .with_header("x-api-key", "1234")
        .with_body(include_str!("../res/link_node.html"))
        .create()
}

/// Tests a real crawl, involving self-referential and cross-referential links.
#[tokio::test]
async fn test_crawl() {
    let _m1 = simple_html();
    let _m2 = self_ref_html();
    let _m3 = link_node_html();

    let client = Client::new();
    let url = mock_url("simple.html");

    use std::vec::Vec;
    use stream::StreamExt;
    let results: Vec<CrawlResult> = crawl_domain_with_client(client, url).collect().await;
    assert_eq!(results.iter().count(), 3);
}

/// Tests the vector of URLs generated from the stream.
#[tokio::test]
async fn unique_url_list() {
    let _m1 = simple_html();
    let _m2 = self_ref_html();
    let _m3 = link_node_html();

    let client = Client::new();
    let url = mock_url("simple.html");
    let list = unique_url_list_with_client(client, url).await;
    assert_eq!(list.iter().count(), 3);
}

/// Tests the count of URLs generated from the stream.
#[tokio::test]
async fn unique_url_count() {
    let _m1 = simple_html();
    let _m2 = self_ref_html();
    let _m3 = link_node_html();

    let client = Client::new();
    let url = mock_url("simple.html");
    let count = unique_url_count_with_client(client, url).await;
    assert_eq!(count, 3);
}
