use std::fmt::format;
use opentelemetry::KeyValue;
use opentelemetry::trace::{Span, Tracer};

// Try to simulate the situation where the UDP packet is too large.
#[tokio::main]
async fn main() {
    let tracer = opentelemetry_jaeger::new_pipeline()
        // .with_auto_split_batch(true)
        .install_batch(opentelemetry::runtime::Tokio).unwrap();

    let mut very_long_attributes = vec![];
    for i in 0..10 {
        very_long_attributes.push(KeyValue::new(format!("key-{}", i), "values"))
    }

    for i in 1..90 {
        let mut span = tracer.start(format!("span-{}", i));
        span.add_event("event_1", very_long_attributes.clone());
        println!("send {}th span", i)
    }

    opentelemetry::global::shutdown_tracer_provider();
}
