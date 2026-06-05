use axum::{Router, routing::get};

async fn handler() -> &'static str {
    "Hello, world!"
}

fn get_router() -> Router {
    Router::new().route("/hello", get(handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_router() {
        let app = get_router();

        let response = app
            .oneshot(
                http::Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world!");
    }
}
