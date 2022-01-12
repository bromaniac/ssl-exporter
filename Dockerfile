FROM debian:bullseye-slim
COPY ./target/release/ssl-expiration /app/ssl-expiration
ENTRYPOINT ["/app/ssl-expiration"]