use cached::proc_macro::cached;
use prometheus_exporter_base::prelude::*;
use ssl_expiration::SslExpiration;
use std::env;
use std::io::{stderr, Write};
use std::process::exit;

#[derive(Debug, Clone, Default)]
struct MyOptions {}

#[tokio::main]
async fn main() {
    let domain = match env::var("SSL_EXPIRATION_DOMAIN") {
        Ok(domain) => domain,
        Err(e) => {
            eprintln!("Couldn't read SSL_EXPIRATION_DOMAIN ({})", e);
            exit(-1)
        }
    };

    // black magic to turn String -> &'static str
    let arg = Box::new(domain);
    let arg: &'static str = Box::leak(arg);

    let addr = ([0, 0, 0, 0], 8080).into();
    println!("starting exporter on {}", addr);

    render_prometheus(
        addr,
        MyOptions::default(),
        move |request, options| async move {
            println!(
                "in our render_prometheus(request == {:?}, options == {:?})",
                request, options
            );

            Ok(PrometheusMetric::build()
                .with_name("days_until_expiry")
                .with_metric_type(MetricType::Counter)
                .with_help("Days left until expiry")
                .build()
                .render_and_append_instance(
                    &PrometheusInstance::new()
                        .with_label("domain", arg)
                        .with_value(check_domain(arg))
                        .with_current_timestamp()
                        .expect("error getting the UNIX epoch"),
                )
                .render())
        },
    )
    .await;

    exit(0);
}

// 24h cache
#[cached(time = 86400, time_refresh = false)]
fn check_domain(domain: &'static str) -> i32 {
    match SslExpiration::from_domain_name(&domain) {
        Ok(expiration) => {
            let days = expiration.days();
            println!("{} SSL certificate will expire in {} days", domain, days);
            days
        }
        Err(e) => {
            let _ = writeln!(
                stderr(),
                "An error occured when checking {}: {}",
                domain,
                e.description()
            );
            -1 // indicates error
        }
    }
}
