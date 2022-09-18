use anyhow::Result;
use opentelemetry::runtime;
use opentelemetry::sdk::{trace, Resource};
use std::sync::Arc;
use tracing::{error, Level, Subscriber};

use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{reload, Layer};

pub struct TracingToggle {
    enable: Box<dyn Fn(String) -> Result<()> + Send + Sync + 'static>,
    disable: Box<dyn Fn() -> Result<()> + Send + Sync + 'static>,
}

#[allow(clippy::missing_errors_doc)]
impl TracingToggle {
    pub fn enable(&self, otlp_endpoint: String) -> Result<()> {
        (self.enable)(otlp_endpoint)
    }

    pub fn disable(&self) -> Result<()> {
        (self.disable)()
    }
}

fn init_tracing<S>(otlp_endpoint: String) -> Result<impl Layer<S>>
where
    for<'span> S: Subscriber + LookupSpan<'span>,
{
    opentelemetry::global::set_error_handler(|error| {
        error!(target: "opentelemetry", ?error);
    })?;

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)?;

    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}

fn disable_tracing() -> Result<()> {
    opentelemetry::global::set_error_handler(|_| {})?;
    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}

fn init_logging(otlp_endpoint: Option<String>) -> Result<TracingToggle> {
    let opentelemetry = otlp_endpoint.map(init_tracing).transpose()?;
    let (opentelemetry, handle) = reload::Layer::new(opentelemetry);
    let handle2 = handle.clone();

    let enable = move |endpoint: String| {
        println!("enabling tracing");
        let layer = init_tracing(endpoint)?;
        println!("enabled tracing");
        handle2.reload(layer)?;
        anyhow::Ok(())
    };

    let disable = move || {
        println!("disabling tracing");
        opentelemetry::global::shutdown_tracer_provider();
        println!("disabled tracing");
        anyhow::Ok(())
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(opentelemetry)
        .with(
            Targets::new()
                .with_target(env!("CARGO_PKG_NAME"), Level::TRACE)
                .with_target("tower_http", Level::TRACE)
                .with_default(Level::INFO),
        )
        .init();

    Ok(TracingToggle {
        enable: Box::new(enable),
        disable: Box::new(disable),
    })
}

#[tokio::main]
async fn main() {
    let toggle = Arc::new(init_logging(Some("http://localhost:4197".to_string())).unwrap());
    let toggle1 = toggle.clone();
    let toggle2 = toggle.clone();

    let enable = tokio::task::spawn_blocking(move || {
        toggle1.enable("http://localhost:4317".to_string()).unwrap();
    });

    let disable = tokio::task::spawn_blocking(move || {
        toggle2.disable().unwrap();
    });

    tokio::join!(enable, disable);
    println!("end of operations");
}
