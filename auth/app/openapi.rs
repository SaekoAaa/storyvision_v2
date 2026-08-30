use auth_service::constants::ROUTER_VERSION_PATH;
use utoipa::{
    OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use {axum::Router, utoipa::Modify};

#[derive(OpenApi)]
#[openapi(info(version = "1.0.0", title = "Storyvision"), tags(
    (name = "auth", description = "Authorization management endpoints")
), modifiers(&SecurityAddon))]
struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("refresh_token"))),
        )
    }
}

pub fn init_openapi(router: Router) -> Router {
    let mut openapi = ApiDoc::openapi();
    openapi = openapi.nest(
        ROUTER_VERSION_PATH,
        auth_service::features::AuthOpenApi::openapi(),
    );
    router.merge(
        utoipa_rapidoc::RapiDoc::with_openapi("/api-docs/openapi.json", openapi).path("/rapidoc"),
    )
}
