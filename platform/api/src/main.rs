use std::net::SocketAddr;

use axum::{Extension, Router};
use dotenv::dotenv;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};

mod middleware;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = database::connection::new()
        .await
        .expect("DB Connection Failed!");

    let app: Router = Router::new()
        .nest("/api/v1", routes::router::api())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(Extension(database))
        .layer(CorsLayer::new().allow_credentials(true));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
