use axum::{
    routing::{get, post},
    Router,
};
use controler::{get_info_handler, login_handler};

mod controler;
mod model;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new()
        .route("/login", post(login_handler))
        .route("/get_info", get(get_info_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
