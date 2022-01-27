use std::net::{SocketAddr};
use std::sync::Arc;

use axum::AddExtensionLayer;
use tokio::signal;
use crate::store::Store;

mod user;
mod route;
mod code;

pub async fn run(store: Arc<Store>, listen_http: &str) {
    let app = route::setup_router();
    let app = app.layer(AddExtensionLayer::new(store));

    let addr: SocketAddr = listen_http.parse().unwrap();
    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
