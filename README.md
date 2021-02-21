# Webcrawler Demo

Built with `rust version 1.50.0 (cb75ad5db 2021-02-10)`

```bash
> curl --header "Content-Type: application/json" -X POST --data '"URL"' localhost:3030/crawler/domains
DOMAIN
```

```bash
> curl -X GET localhost:3030/crawler/domains/DOMAIN/urls
{"crawl_completed":[true|false],"urls":[...]}
```

```bash
> curl -X GET localhost:3030/crawler/domains/DOMAIN/urls/count
{"crawl_completed":[true|false],"url_count":COUNT}
```