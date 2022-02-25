# Update 2022-02-22:
As of version 1.3.7 the [cert-util-operator](https://github.com/redhat-cop/cert-utils-operator) supports alerting on expiring certs so this is probably not needed anymore.

# ssl-expiration

Checks SSL certificate expiration, exports as Prometheus metric and alerts when expiry is near. (I wrote this for Openshift but it should work in vanilla Kubernetes. I haven't tested it.)

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
BUILD_KIT=1 docker build -t ssl-exporter .
```
