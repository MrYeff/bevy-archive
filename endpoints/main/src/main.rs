mod article;
mod root;

use axum::{Router, routing::*};
use tokio::net::TcpListener;

const URL: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root::handler))
        .nest("/article", article::route());

    let listener = TcpListener::bind(URL).await.unwrap();
    println!("Listening on http://{}", URL);
    axum::serve(listener, app).await.unwrap();
}
