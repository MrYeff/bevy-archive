mod preview;
mod register;
mod root;

use axum::{Router, routing::*};

use crate::Ctx;

pub(super) fn route() -> Router<Ctx> {
    Router::new()
        .route("/", get(root::handler))
        .route("/{temp_id}/register", post(register::handler))
        .route("/{temp_id}/preview", get(preview::handler))
}
