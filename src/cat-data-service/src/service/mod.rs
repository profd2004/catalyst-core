use crate::state::State;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use poem::{http::Method, listener::TcpListener, middleware::Cors, Endpoint, EndpointExt, Route};
use poem_openapi::{
    types::{ToJSON, Type},
    ApiResponse, Object, OpenApiService,
};
use serde::Serialize;
use std::{future::ready, net::SocketAddr, sync::Arc, time::Instant};

#[allow(dead_code)]
mod health;
#[allow(dead_code)]
mod v0;
#[allow(dead_code)]
mod v1;

const API_NAME: &str = "Catalyst 1.0";
const API_VERSION: &str = "1.0.0";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot run service, error: {0}")]
    CannotRunService(String),
    #[error(transparent)]
    EventDbError(#[from] event_db::error::Error),
}

#[derive(Serialize, Debug, Object)]
struct ErrorMessage {
    error: String,
}

#[derive(ApiResponse)]
enum PoemResponse<T: Send + Type + ToJSON> {
    #[oai(status = 200)]
    Ok(poem_openapi::payload::Json<T>),

    #[oai(status = 404)]
    NotFound(poem_openapi::payload::Json<ErrorMessage>),

    #[oai(status = 500)]
    InternalServerError(poem_openapi::payload::Json<ErrorMessage>),
}

fn handle_result<T: Send + Type + ToJSON>(res: Result<T, Error>) -> PoemResponse<T> {
    match res {
        Ok(res) => PoemResponse::Ok(poem_openapi::payload::Json(res)),
        Err(Error::EventDbError(event_db::error::Error::NotFound(error))) => {
            PoemResponse::NotFound(poem_openapi::payload::Json(ErrorMessage { error }))
        }
        Err(error) => {
            PoemResponse::InternalServerError(poem_openapi::payload::Json(ErrorMessage {
                error: error.to_string(),
            }))
        }
    }
}

pub fn app(_state: Arc<State>) -> poem::Route {
    // build our application with a route
    let health_api = health::HealthApi;
    let api_service =
        OpenApiService::new(health_api, API_NAME, API_VERSION).server("http://localhost:3000");
    Route::new().nest("", api_service)
}

#[allow(dead_code)]
pub fn axum_app(state: Arc<State>) -> axum::Router {
    // build our application with a route
    let v0 = v0::v0(state.clone());
    let v1 = v1::v1(state);
    let health = health::health();
    axum::Router::new().nest("", v1.merge(v0)).merge(health)
}

#[allow(dead_code)]
fn metrics_app() -> axum::Router {
    let recorder_handle = setup_metrics_recorder();
    axum::Router::new().route(
        "/metrics",
        axum::routing::get(move || ready(recorder_handle.render())),
    )
}

fn cors_layer() -> Cors {
    // Allow ANY origin and ANY header
    Cors::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("*")
        .allow_header("*")
}

async fn run_service<E: Endpoint + 'static>(
    app: E,
    addr: &SocketAddr,
    name: &str,
) -> Result<(), Error> {
    tracing::info!("Starting {name}...");
    tracing::info!("Listening on {addr}");

    poem::Server::new(TcpListener::bind(addr))
        .run(app)
        .await
        .map_err(|e| Error::CannotRunService(e.to_string()))?;
    Ok(())
}

pub async fn run(
    service_addr: &SocketAddr,
    metrics_addr: &Option<SocketAddr>,
    state: Arc<State>,
) -> Result<(), Error> {
    let cors = cors_layer();
    if let Some(_metrics_addr) = metrics_addr {
        // let service_app = axum_app(state)
        //     .layer(cors.clone())
        //     .route_layer(axum::middleware::from_fn(track_metrics));
        // let metrics_app = metrics_app().layer(cors);

        // tokio::try_join!(
        //     run_service(service_app, service_addr, "service"),
        //     run_service(metrics_app, metrics_addr, "metrics"),
        // )?;

        let service_app = app(state).with(cors);

        run_service(service_app, service_addr, "service").await?;
    } else {
        let service_app = app(state).with(cors);

        run_service(service_app, service_addr, "service").await?;
    }
    Ok(())
}

fn axum_handle_result<T: Serialize>(res: Result<T, Error>) -> axum::response::Response {
    use axum::response::IntoResponse;
    match res {
        Ok(res) => (axum::http::StatusCode::OK, axum::Json(res)).into_response(),
        Err(Error::EventDbError(event_db::error::Error::NotFound(error))) => (
            axum::http::StatusCode::NOT_FOUND,
            axum::Json(ErrorMessage { error }),
        )
            .into_response(),
        Err(error) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(ErrorMessage {
                error: error.to_string(),
            }),
        )
            .into_response(),
    }
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

#[allow(dead_code)]
async fn track_metrics<T>(
    req: axum::http::Request<T>,
    next: axum::middleware::Next<T>,
) -> impl axum::response::IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<axum::extract::MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    pub fn body_data_json_check(body_data: Vec<u8>, expected_json: serde_json::Value) -> bool {
        serde_json::Value::from_str(String::from_utf8(body_data).unwrap().as_str()).unwrap()
            == expected_json
    }
}
