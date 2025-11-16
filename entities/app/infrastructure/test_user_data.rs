use axum::{
    Router,
    extract::Request,
    middleware::{Next, from_fn},
};
use entities_service::features::common::UserData;

pub fn insert_test_user_data(router: Router) -> Router {
    let router = router.layer(from_fn(async move |mut req: Request, next: Next| {
        let test_user_data = UserData {
            id: 1,
            role: "admin".to_string(),
            projects_list: vec![1, 2, 3],
        };
        let extensions = req.extensions_mut();
        extensions.insert(test_user_data);
        let response = next.run(req).await;
        response
    }));
    router
}
