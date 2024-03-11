use axum::{
    extract::{OriginalUri, Request},
    http::StatusCode,
    routing::get,
    Json, RequestPartsExt, Router,
};
use serde::Serialize;
use tower::util::ServiceExt;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct Time {
    current: String,
}

fn get_time() -> Time {
    Time {
        current: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string(),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let public_path = std::env!("CARGO_MANIFEST_DIR").to_owned() + "/dist";
    // println!("{}", public_path); for debugging
    let fallback_service = ServeDir::new(public_path).append_index_html_on_directories(true);

    let app = Router::new()
        .fallback(get(|req: Request| async move {
            let (mut parts, body) = req.into_parts();
            let uri: OriginalUri = parts.extract().await?;

            tracing::info!("Request path: {}", uri.path());
            let req = Request::from_parts(parts, body);
            match fallback_service.oneshot(req).await {
                Ok(mut res) => match res.status() {
                    StatusCode::OK => {
                        if uri.path().contains("/_astro/") {
                            res.headers_mut().insert(
                                "Cache-Control",
                                "public, max-age=31536000".parse().unwrap(),
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
        .route("/api/", get(|| async { "Hello, World!!!" }))
        .route("/api/time/", get(|| async { Json(get_time()) }));

    // run the app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
