use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::app::state::AppState;
use crate::handlers::{
    health::health::__path_get_health,
    bibles::bibles::__path_get_bibles
};

#[derive(OpenApi)]
#[openapi(paths(get_health, get_bibles))]
struct ApiDoc;

pub fn get_swagger_route() -> Router<AppState> {
    SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()).into()
}