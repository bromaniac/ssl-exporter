// The MIT License (MIT)
//
// Copyright (c) 2022 Fredrik Broman <fredrik.broman@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
            exit(22) // EINVAL
        }
    };

    let addr = ([0, 0, 0, 0], 8080).into();
    println!("starting exporter on {}", addr);

    render_prometheus(addr, MyOptions::default(), |request, options| async move {
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
                    .with_label("domain", &*domain)
                    .with_value(check_domain(&domain))
                    .with_current_timestamp()
                    .expect("error getting the UNIX epoch"),
            )
            .render())
    })
    .await;

    exit(0);
}

fn check_domain(domain: &str) -> i32 {
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
