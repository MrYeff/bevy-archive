mod content;
mod new;
mod root;

use axum::{Router, routing::*};

pub(super) fn route() -> Router {
    Router::new()
        .route("/{id}", get(root::handler))
        .route("/{id}/content", get(content::handler))
        .nest("/new", new::route())
}
