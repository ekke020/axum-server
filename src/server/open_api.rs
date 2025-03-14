use swagger_ui_dist::{ApiDefinition, OpenApiSource};
use axum::routing::get;
use super::Router;

const API_SPEC: &str = include_str!("../../api/openapi.yaml");
const API_DEF: ApiDefinition<&str> = ApiDefinition {
    uri_prefix: "/swagger-ui",
    api_definition: OpenApiSource::Uri("/openapi.yml"),
    title: Some("My Super Duper API"),
};

pub fn generate_swagger_ui_routes() -> Router {
    Router::new()
        .route(
            "/openapi.yml",
            get(|| async move { API_SPEC }),
        )
        .merge(swagger_ui_dist::generate_routes(API_DEF))
}