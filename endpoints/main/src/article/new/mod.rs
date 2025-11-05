mod load_preview;
mod preview;
mod register;
mod root;

use askama::Template;
use axum::{Router, routing::*};

pub(super) fn route() -> Router {
    Router::new()
        .route("/", get(root::handler))
        .route("/{temp_id}/register", post(register::handler))
        .route("/{temp_id}/preview", get(preview::handler))
        .route("/load_preview", get(load_preview::handler))
}
