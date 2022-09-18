# How to swap the tracer provider in runtime.

## Overview
This examples shows how to swap the tracer provider during runtime and shutdown the previous provider.
The use case of this includes:
- Prevent span collection(although users should consider Sampler first)
- Shut down span export

## Investigate notes
Issue was first reported in https://github.com/open-telemetry/opentelemetry-rust/issues/868. 

The user reports the tracer provider cannot be shutdown and provided the example used here.

We further developed two function to reproduce the issue:
- `init_logging_works` shuts down tracer provider normally
- `init_logging_buged` fails to shut down tracer provider and hang forever.

Potential reason: 
- We rule out any tracing component and what we observe is if we set the tracer provider concurrently there is a bug.
```rust
let _ = opentelemetry::global::set_tracer_provider(tracer_provider);
// in another thread
let _ = opentelemetry::global::set_tracer_provider(tracer_provider);

```

Observation:
- OTLP doesn't matter here, we get the same bug using stdout exporter.
- Build pipeline doesn't matter here, we get the same bug without pipeline