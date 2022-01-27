#![allow(dead_code)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate rbatis;

use std::sync::Arc;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use futures::executor::block_on;
use rbatis::rbatis::Rbatis;
use serde_derive::Deserialize;

#[macro_use]
mod server;
mod store;

#[derive(Debug, Deserialize)]
struct Config {
    listen_http: String,
    mysql_url: String,
}

impl Config {
    fn new(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut c = config::Config::new();
        c.set_default("listen_http", "127.0.0.1:8080")?;
        c.set_default("mysql_url", "mysql://root:123456@127.0.0.1:3306/test")?;
        c.merge(config::File::with_name(file))?;
        c.merge(config::Environment::with_prefix("REST_API_SERVER"))?;
        Ok(c.try_into()?)
    }
}

async fn async_main(c: Config) {
    let rb = Rbatis::new();
    rb.link(c.mysql_url.as_str()).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);
    let store = Arc::new(store::Store::new(rb));

    let f1 = server::run(store.clone(), c.listen_http.as_str());
    let f2 = server::run(store.clone(), "127.0.0.1:8999");
    futures::join!(f1,f2);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting up");
    let opts = app_from_crate!()
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Configuration file path")
                .takes_value(true)
                .default_value("./config/app.yaml"),
        )
        .get_matches();
    let cfg = Config::new(opts.value_of("config").unwrap())?;

    block_on(async_main(cfg));
    Ok(())
}
