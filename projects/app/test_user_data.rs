use axum::{
    Router,
    extract::Request,
    middleware::{Next, from_fn},
};
use projects_service::features::common::UserData;

// Used to bypass JWT verification
pub fn insert_test_user_data(router: Router) -> Router {
    router.layer(from_fn(async move |mut req: Request, next: Next| {
        let test_user_data = UserData {
            id: 1,
            role: "admin".to_string(),
        };
        let extensions = req.extensions_mut();
        extensions.insert(test_user_data);
        next.run(req).await
    }))
}
