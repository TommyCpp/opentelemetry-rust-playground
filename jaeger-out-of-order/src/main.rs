use opentelemetry::sdk::export::trace::{SpanData, SpanExporter};
use opentelemetry::sdk::trace::{EvictedHashMap, EvictedQueue};
use opentelemetry::trace::{
    SpanContext, SpanId, SpanKind, StatusCode, TraceFlags, TraceId, TraceState,
};
use std::ops::{Add, Sub};
use std::time::{Duration, SystemTime};

fn main() {
    let mut exporter = opentelemetry_jaeger::new_pipeline()
        .with_service_name("test service")
        .init_sync_exporter()
        .unwrap();

    let trace_id = TraceId::from([1, 3, 4, 5, 1, 2, 3, 4, 1, 3, 4, 5, 1, 2, 3, 4]);

    let start_time = SystemTime::now();

    let parent_span = SpanData {
        span_context: SpanContext::new(
            trace_id,
            SpanId::from([1, 1, 1, 1, 1, 1, 1, 1]),
            TraceFlags::SAMPLED,
            false,
            TraceState::default(),
        ),
        parent_span_id: SpanId::INVALID,
        span_kind: SpanKind::Client,
        name: "first span".into(),
        start_time: start_time.sub(Duration::from_secs(4)),
        end_time: SystemTime::now().add(Duration::from_secs(20)),
        attributes: EvictedHashMap::new(12, 24),
        events: EvictedQueue::new(12),
        links: EvictedQueue::new(12),
        status_code: StatusCode::Unset,
        status_message: Default::default(),
        resource: None,
        instrumentation_lib: Default::default(),
    };

    let span = SpanData {
        span_context: SpanContext::new(
            trace_id,
            SpanId::from([1, 2, 3, 4, 5, 6, 7, 8]),
            TraceFlags::SAMPLED,
            false,
            TraceState::default(),
        ),
        parent_span_id: SpanId::from([1, 1, 1, 1, 1, 1, 1, 1]),
        span_kind: SpanKind::Client,
        name: "second span".into(),
        start_time: start_time,
        end_time: SystemTime::now().add(Duration::from_secs(20)),
        attributes: EvictedHashMap::new(12, 24),
        events: EvictedQueue::new(12),
        links: EvictedQueue::new(12),
        status_code: StatusCode::Unset,
        status_message: Default::default(),
        resource: None,
        instrumentation_lib: Default::default(),
    };

    let exporter_ref = &mut exporter;
    futures_executor::block_on(async move { exporter_ref.export(vec![span]).await });

    let exporter_ref = &mut exporter;
    futures_executor::block_on(async move {
        exporter_ref.export(vec![parent_span]).await
    });
}
