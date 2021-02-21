# Build stage
FROM rust:1.50.0 as builder
WORKDIR /usr/src/webcrawler_demo
# Copy src files
COPY . .
# build app
RUN cargo build --release

# Release stage
FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl
EXPOSE 8080
# copy the build artifact from the build stage
COPY --from=builder /usr/src/webcrawler_demo/target/release/web_crawler_server /usr/local/bin/web_crawler_server
# Run the web crawler
CMD web_crawler_server "0.0.0.0:8080"
