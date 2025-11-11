mod content;
mod new;
mod root;

use axum::{Router, routing::*};

use crate::Ctx;

pub(super) fn route() -> Router<Ctx> {
    Router::new()
        .route("/{id}", get(root::handler))
        .route("/{id}/content", get(content::handler))
        .nest("/new", new::route())
}
