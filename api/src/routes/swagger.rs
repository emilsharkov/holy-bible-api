use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::app::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(),
    tags(
        (name = "example", description = "Example API")
    )
)]
struct ApiDoc;

pub fn get_swagger_route() -> Router<AppState> {
    SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()).into()
}