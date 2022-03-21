use std::fmt::Debug;
use opentelemetry::sdk::export::trace::{ExportResult, SpanData, SpanExporter};
use opentelemetry_datadog::{ApiVersion, DatadogExporter, DatadogPipelineBuilder, new_pipeline};
use async_trait::async_trait;
use opentelemetry::trace::SpanKind;

fn get_datadog_pipeline() -> DatadogPipelineBuilder {
    new_pipeline()
        .with_agent_endpoint("http://localhost:8126")
        .with_version(ApiVersion::Version03)
}

// contains multiple exporters associated with different service name
#[derive(Debug)]
struct CustomExporter {
    current_service: DatadogExporter,
    dependencies: DatadogExporter,
}

#[async_trait]
impl SpanExporter for CustomExporter {
    async fn export(&mut self, batch: Vec<SpanData>) -> ExportResult {
        let mut current_service_spans = Vec::with_capacity(batch.len() / 2);
        let mut dependencies_spans = Vec::with_capacity(batch.len() / 2);
        for span in batch.into_iter() {
            if span.span_kind == SpanKind::Client { // use any span information to determine which service name/exporter to bind to the span.
                dependencies_spans.push(span)
            } else {
                current_service_spans.push(span)
            }
        }
        self.current_service.export(current_service_spans).await?;
        self.dependencies.export(dependencies_spans).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {

    let exporter1 = get_datadog_pipeline().with_service_name("service1").build_exporter().unwrap();
    let exporter2 = get_datadog_pipeline().with_service_name("service2").build_exporter().unwrap();

    let exporter = CustomExporter {
        current_service: exporter1,
        dependencies: exporter2,
    };

    // set config in tracer provide builder if needed.
    let provider =
        opentelemetry::sdk::trace::TracerProvider::builder()
            .with_batch_exporter(exporter, opentelemetry::runtime::Tokio)
            .build();

    // set the provide into otel context
    opentelemetry::global::set_tracer_provider(provider);

    let _test_tracer = opentelemetry::global::tracer("test_tracer"); // the `resource` tag will be test_tracer
    let _dependency_tracer = opentelemetry::global::tracer("dependency_tracer"); // the `resource` tag will be dependency_tracer
}