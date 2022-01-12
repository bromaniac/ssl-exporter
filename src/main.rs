use ssl_expiration::SslExpiration;
use std::env;
use std::io::{stderr, Write};
use std::process::exit;

fn main() {
    let domains_env = match env::var("SSL_EXPIRATION_DOMAINS") {
        Ok(domains) => domains,
        Err(e) => {
            eprintln!("Couldn't read SSL_EXPIRATION_DOMAINS ({})", e);
            exit(-1)
        }
    };

    let domains = domains_env.split(",");

    for domain in domains {
        match SslExpiration::from_domain_name(&domain) {
            Ok(expiration) => {
                let days = expiration.days();
                println!("{} SSL certificate will expire in {} days", domain, days);
                if days < 22 {
                    alert();
                }
            }
            Err(e) => {
                let _ = writeln!(
                    stderr(),
                    "An error occured when checking {}: {}",
                    domain,
                    e.description()
                );
            }
        }
    }
}

fn alert() {
    unimplemented!()
}