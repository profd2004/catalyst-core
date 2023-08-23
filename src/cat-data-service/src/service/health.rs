use super::{axum_handle_result, handle_result, PoemResponse};
use crate::service::Error;
use axum::{routing::get, Router};
use poem_openapi::OpenApi;

pub struct HealthApi;

#[OpenApi]
impl HealthApi {
    #[oai(path = "/health/ready", method = "get")]
    async fn health_ready(&self) -> PoemResponse<bool> {
        tracing::debug!("health ready exec");

        handle_result(Ok(true))
    }

    #[oai(path = "/health/live", method = "get")]
    async fn health_live(&self) -> PoemResponse<bool> {
        tracing::debug!("health live exec");

        handle_result(Ok(true))
    }
}

pub fn health() -> Router {
    Router::new()
        .route(
            "/health/ready",
            get(|| async { axum_handle_result(ready_exec().await) }),
        )
        .route(
            "/health/live",
            get(|| async { axum_handle_result(live_exec().await) }),
        )
}

async fn ready_exec() -> Result<bool, Error> {
    tracing::debug!("health ready exec");

    Ok(true)
}

async fn live_exec() -> Result<bool, Error> {
    tracing::debug!("health live exec");

    Ok(true)
}

/// Need to setup and run a test event db instance
/// To do it you can use the following commands:
/// Prepare docker images
/// ```
/// earthly ./containers/event-db-migrations+docker --data=test
/// ```
/// Run event-db container
/// ```
/// docker-compose -f src/event-db/docker-compose.yml up migrations
/// ```
/// Also need establish `EVENT_DB_URL` env variable with the following value
/// ```
/// EVENT_DB_URL="postgres://catalyst-event-dev:CHANGE_ME@localhost/CatalystEventDev"
/// ```
/// https://github.com/input-output-hk/catalyst-core/tree/main/src/event-db/Readme.md
#[cfg(test)]
mod tests {
    use crate::{service::axum_app, state::State};
    use axum::{
        body::{Body, HttpBody},
        http::{Request, StatusCode},
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_ready_test() {
        let state = Arc::new(State::new(None).await.unwrap());
        let app = axum_app(state);

        let request = Request::builder()
            .uri("/health/ready".to_string())
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            String::from_utf8(response.into_body().data().await.unwrap().unwrap().to_vec())
                .unwrap()
                .as_str(),
            r#"true"#
        );
    }

    #[tokio::test]
    async fn health_live_test() {
        let state = Arc::new(State::new(None).await.unwrap());
        let app = axum_app(state);

        let request = Request::builder()
            .uri("/health/live".to_string())
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            String::from_utf8(response.into_body().data().await.unwrap().unwrap().to_vec())
                .unwrap()
                .as_str(),
            r#"true"#
        );
    }
}
