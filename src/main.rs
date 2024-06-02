use axum::Router;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        // .route("/", get(hello_world))
        .nest_service("/", ServeDir::new("frontend/dist"));

    Ok(router.into())
}
