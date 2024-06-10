use anyhow::{Context, Result};
use axum::{http::{Response, StatusCode}, response::{Html, IntoResponse}, routing::get, Router};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(|| async { Html(include_str!("./html/index.html")) }))
        .route("/style.css", get(css_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Could not bind TcpListener")?;

    println!("Listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    println!("Shutting down...");
    Ok(())
}

async fn css_handler() -> impl IntoResponse {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(String::from(include_str!("./html/style.css")))
        .unwrap();

    response
}

// Code borrowed from https://github.com/tokio-rs/axum/blob/806bc26e62afc2e0c83240a9e85c14c96bc2ceb3/examples/graceful-shutdown/src/main.rs
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
}

