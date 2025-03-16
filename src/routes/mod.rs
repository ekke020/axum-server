use axum::response::IntoResponse;
use http::StatusCode;
use axum::Router;
mod swagger;

pub fn routes() -> Router {
    Router::new()
        .merge(swagger::ui_routes())
        .fallback(handler_404)
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}