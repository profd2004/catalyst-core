//! Poem Service for cat-data-service endpoints.
//!
//! This provides only the primary entrypoint to the service.

use crate::service::docs::docs;
use crate::service::Error;

use crate::service::api::mk_api;
use crate::service::generic::responses::resp_5xx::ServerError;
use crate::service::utilities::metrics_tracing::{init_prometheus, log_requests};

use panic_message::panic_message;
use poem::endpoint::PrometheusExporter;
use poem::listener::TcpListener;
use poem::middleware::{CatchPanic, Cors, OpenTelemetryMetrics, PanicHandler};
use poem::{EndpointExt, Route};
use std::any::Any;
use std::cell::RefCell;
use std::net::SocketAddr;
use tracing::log::error;

use std::backtrace::Backtrace;

/// Customized Panic handler.
/// Catches all panics, and turns them into 500.
/// Does not crash the service, BUT will set it to NOT LIVE.
/// Logs the panic as an error.
/// This should cause Kubernetes to restart the service.
#[derive(Clone)]
struct ServicePanicHandler;

// Customized Panic handler - data storage.
// Allows us to catch the backtrace so we can include it in logs.
thread_local! {
    static BACKTRACE: RefCell<Option<String>> = RefCell::new(None);
    static LOCATION: RefCell<Option<String>> = RefCell::new(None);
}

/// Sets a custom panic hook to capture the Backtrace and Panic Location for logging purposes.
/// This hook gets called BEFORE we catch it.  So the thread local variables stored here are
/// valid when processing the panic capture.
fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        // Get the backtrace and format it.
        let raw_trace = Backtrace::force_capture();
        let trace = format!("{raw_trace}");
        BACKTRACE.with(move |b| b.borrow_mut().replace(trace));

        // Get the location and format it.
        let location = match panic_info.location() {
            Some(location) => format!("{location}"),
            None => "Unknown".to_string(),
        };
        LOCATION.with(move |l| l.borrow_mut().replace(location));
    }));
}

impl PanicHandler for ServicePanicHandler {
    type Response = ServerError;

    fn get_response(&self, err: Box<dyn Any + Send + 'static>) -> ServerError {
        let response = ServerError::new(Some(
            "Internal Server Error.  Please report the issue to the service owner.".to_string(),
        ));

        // Get the unique identifier for this panic, so we can find it in the logs.
        let panic_identifier = response.id().to_string();

        // Get the message from the panic as best we can.
        let err_msg = panic_message(&err);

        // This is the location of the panic.
        let location = match LOCATION.with(|l| l.borrow_mut().take()) {
            Some(location) => location,
            None => "Unknown".to_string(),
        };

        // This is the backtrace of the panic.
        let backtrace = match BACKTRACE.with(|b| b.borrow_mut().take()) {
            Some(backtrace) => backtrace,
            None => "Unknown".to_string(),
        };

        error!(
            panic = panic_identifier,
            error = err_msg,
            loc = location,
            bt = backtrace;
            "PANIC"
        );

        response
    }
}

/// Run the Poem Service
///
/// This provides only the primary entrypoint to the service.
/// addr: &SocketAddr - the address to listen on
///
pub async fn run_service(addr: &SocketAddr) -> Result<(), Error> {
    tracing::info!("Starting Poem Service ...");
    tracing::info!("Listening on {addr}");

    // Set a custom panic hook, so we can catch panics and not crash the service.
    // And also get data from the panic so we can log it.
    // Panics will cause a 500 to be sent with minimal information we can use to
    // help find them in the logs if they happen in production.
    set_panic_hook();

    let api_service = mk_api(addr);
    let docs = docs(&api_service);

    let prometheus_controller = init_prometheus();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs", docs)
        .nest(
            "/metrics",
            PrometheusExporter::with_controller(prometheus_controller),
        )
        .with(Cors::new())
        .with(OpenTelemetryMetrics::new())
        .with(CatchPanic::new().with_handler(ServicePanicHandler))
        .around(|ep, req| async move { Ok(log_requests(ep, req).await) });

    poem::Server::new(TcpListener::bind(addr))
        .run(app)
        .await
        .map_err(Error::Io)
}
