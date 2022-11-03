use axum::{routing::{get}, Router};

pub fn router() -> Router{
    let router: Router = Router::new()
        .route("/" ,get( || async {"Hello, World!" }));
    return router;
}