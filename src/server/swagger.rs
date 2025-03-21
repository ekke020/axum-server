use axum::{routing::get, Router};
use swagger_ui_dist::{ApiDefinition, OpenApiSource};

const API_SPEC: &str = include_str!("../../api/openapi.yaml");
const TAG_MODELS: &str = include_str!("../../api/tag-models.yaml");
const TAG_API: &str = include_str!("../../api/tag-api.yaml");
const COMMON_MODELS: &str = include_str!("../../api/common.yaml");
const API_DEF: ApiDefinition<&str> = ApiDefinition {
    uri_prefix: "/swagger-ui",
    api_definition: OpenApiSource::Uri("/openapi.yml"),
    title: Some("Test API"),
};

pub fn ui_routes() -> Router {
    Router::new()
        .route("/openapi.yml", get(|| async move { API_SPEC }))
        .route("/tag-api.yaml", get(|| async move { TAG_API }))
        .route("/tag-models.yaml", get(|| async move { TAG_MODELS }))
        .route("/common.yaml", get(|| async move { COMMON_MODELS }))
        .merge(swagger_ui_dist::generate_routes(API_DEF))
}
