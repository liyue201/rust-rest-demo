#[macro_use]
extern crate log;
#[macro_use]
pub mod server;

use futures::executor::block_on;

async fn async_main() {
    let f1 = server::run(3000);
    let f2 = server::run(3001);
    futures::join!(f1,f2);
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    block_on(async_main())
}
