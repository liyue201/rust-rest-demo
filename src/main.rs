#![allow(dead_code)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate rbatis;

use std::sync::Arc;

use futures::executor::block_on;
use rbatis::rbatis::Rbatis;

#[macro_use]
pub mod server;

pub const MYSQL_URL: &'static str = "mysql://fortest:Ky6XRHMFWScBPpbC@122.9.61.5:3306/fortest";

async fn async_main() {
    let rb = Rbatis::new();
    rb.link(MYSQL_URL).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);
    let f1 = server::run(rb.clone(), 3000);
    let f2 = server::run(rb.clone(), 3001);
    futures::join!(f1,f2);
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");
    block_on(async_main())
}
