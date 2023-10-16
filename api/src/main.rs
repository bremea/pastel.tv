use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};
use dotenv::dotenv;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

mod routes;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database = util::db::connect().await.expect("DB Connection Failed!");

    let app: Router = Router::new()
        .nest("/api/v1", routes::router::api())
        //.route("/gateway/v1", get(gateway::ws::ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(Extension(database));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
