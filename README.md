# ssl-expiration

Checks SSL certificate expiration and exports as Prometheus metric. The check is cached for 24 hours so the domain being checked won't be hammered with requests. (This is moot if you deploy to OCP since you can set scrape interval in the ServiceMonitor.)

## Deploy in Openshift
Review ocp-deploy.yaml and then deploy in Openshift
```bash
oc apply -f ocp-deploy.yaml
```

## Run container locally
```bash
docker pull alpha60/ssl-exporter:latest
docker run --rm -e SSL_EXPIRATION_DOMAIN=github.com -p 80:8080 ssl-exporter
```

In another terminal
```bash
curl localhost/metrics
```

## Run binary locally
```bash
SSL_EXPIRATION_DOMAIN=github.com cargo r
```

In another terminal
```bash
curl localhost:8080/metrics
```

## Build container image
```bash
cargo b --release
strip target/release/ssl-exporter
BUILD_KIT=1 docker build -t ssl-exporter .
```
