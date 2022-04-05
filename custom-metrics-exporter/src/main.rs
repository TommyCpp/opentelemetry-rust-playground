use std::time::Duration;
use futures_util::{Stream, StreamExt};
use opentelemetry::global;
use opentelemetry::metrics::Descriptor;
use opentelemetry::sdk::export::metrics::{
    CheckpointSet, ExportKind, ExportKindFor, ExportKindSelector,
};
use opentelemetry::sdk::metrics::selectors::simple::Selector;

// Skip first immediate tick from tokio, not needed for async_std.
fn delayed_interval(duration: Duration) -> impl Stream<Item = tokio::time::Instant> {
    opentelemetry::util::tokio_interval_stream(duration).skip(1)
}

#[tokio::main]
async fn main() {
    let controller = opentelemetry::sdk::metrics::controllers::push(
        Selector::Inexpensive,
        ExportKindSelector::Cumulative,
        MockExporter{},
        tokio::spawn,
        delayed_interval,
    ).build();
    global::set_meter_provider(controller.provider());

    let counter = global::meter("test").u64_counter("counter1");
}

#[derive(Debug)]
struct MockExporter {}

impl ExportKindFor for MockExporter {
    fn export_kind_for(&self, descriptor: &Descriptor) -> ExportKind {
        ExportKind::Delta
    }
}

impl opentelemetry::sdk::export::metrics::Exporter for MockExporter {
    fn export(&self, checkpoint_set: &mut dyn CheckpointSet) -> opentelemetry::metrics::Result<()> {
        Ok(())
    }
}
