mod api_handlers;

use crate::api_handlers::server_time::get_time;
use axum::{
    extract::{OriginalUri, Request},
    http::StatusCode,
    routing::get,
    RequestPartsExt, Router,
};
use dotenv::dotenv;
use tower::util::ServiceExt;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let api_host = std::env::var("PUBLIC_HOST").unwrap_or_else(|_| {
        println!("\x1b[38;2;217;194;140mWarning\x1b[0m PUBLIC_HOST not set in .env, using localhost:3000 as default");
        "localhost:3000".to_string()
    });

    let public_path = std::env!("CARGO_MANIFEST_DIR").to_owned() + "/dist";
    let fallback_service = ServeDir::new(public_path).append_index_html_on_directories(true);

    let comression_layer: CompressionLayer = CompressionLayer::new().gzip(true);

    let app = Router::new()
        .fallback(get(|req: Request| async move {
            let (mut parts, body) = req.into_parts();
            let uri: OriginalUri = parts.extract().await?;

            tracing::info!("Request path: {}", uri.path());
            let req = Request::from_parts(parts, body);
            match fallback_service.oneshot(req).await {
                Ok(mut res) => match res.status() {
                    StatusCode::OK => {
                        if uri.path().contains("/_static/") {
                            res.headers_mut().insert(
                                "Cache-Control",
                                // One year cache
                                "public, max-age=31536000".parse().unwrap(),
                            );
                        }
                        if uri.path().contains("/_astro/") {
                            res.headers_mut().insert(
                                "Cache-Control",
                                // One month cache
                                "public, max-age=2628000".parse().unwrap(),
                            );
                        }
                        Ok(res)
                    }
                    _ => Ok(res),
                },
                Err(e) => {
                    tracing::error!("fallback_service error: {}", e);
                    Err(e)
                }
            }
        }))
        .layer(comression_layer)
        .route("/api/time/", get(get_time));

    let listener = tokio::net::TcpListener::bind(api_host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
