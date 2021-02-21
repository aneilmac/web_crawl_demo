# Webcrawler Demo

Demo web-crawler server written using Rust and `warp`.

This is a toy application that provides a web API to crawl some arbitrary 
given domain. Only URLs with a `https` or `http` scheme are accepted.

## Usage

A crawl starts by POSTing a URL to `/crawler/domains` like so:

```bash
> curl --header "Content-Type: application/json" -X POST --data '"<URL>"' <HOST>/crawler/domains
<DOMAIN>
```

Where `URL` is some http or https URL for a given domain. 
The crawl of `DOMAIN` will start at the page pointed to by `URL`.

This is an asynchronous process, the crawl is live, and updates constantly until completion.

The returned key `DOMAIN` can be used as a key to lookup the crawled URLs, like so:

```bash
> curl -X GET <HOST>/crawler/domains/<DOMAIN>/urls
{"crawl_completed": [true|false], "urls": [...]}
```

The server returns a JSON object with two elements, a list of crawled URLs in 
the given domain, `DOMAIN`, and the flag `crawl_completed`, indicating whether the 
crawl has finished over the entire domain (true) or is still in progress (false).

The count api for domain, `DOMAIN` has a similar syntax.

```bash
> curl -X GET <HOST>/crawler/domains/<DOMAIN>/urls/count
{"crawl_completed": [true|false], "url_count": <COUNT>}
```

## Build and run

To build and run the debug webserver listening to address `127.0.0.1:8080`:

```
cargo build
cargo run
```

## Build and run tests

To run the full test suite across the application and crawler library:

```
cargo test # Test the application
cargo test -p web_crawler_lib # Test the crawler library
```

## Example

An example of usage using a live domain:

```bash
> curl --header "Content-Type: application/json" -X POST --data '"https://www.enhance.com"' localhost:8080/crawler/domains
www.enhance.com
```

```bash
> curl -X GET localhost:8080/crawler/domains/www.enhance.com/urls
{"crawl_completed": true, "urls": ["https://www.enhance.com/","https://www.enhance.com/styles.208feb938cace1c3135d.css","https://www.enhance.com/favicon.ico"]}
```

```bash
> curl -X GET localhost:8080/crawler/domains/www.enhance.com/urls/count
{"crawl_completed": true, "url_count": 3}
```

## Docker integration

A docker image has been provided. To run the image listening on port 8080:

```bash
docker build --tag webcrawl_server .
docker run --publish 8080:8080 webcrawl_server
```
