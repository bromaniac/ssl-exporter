FROM debian:bullseye-slim
COPY ./target/release/ssl-exporter /app/ssl-exporter
ENTRYPOINT ["/app/ssl-exporter"]