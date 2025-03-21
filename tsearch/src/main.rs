use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    // build app and add single route
    let app = Router::new().route("/", get(|| async { "Hello World!"}));

    // run the app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
