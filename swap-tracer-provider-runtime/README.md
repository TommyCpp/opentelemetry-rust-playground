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
- We use a sync ReadWriteLock to guard the tracer provider. Could it be we hold it across the `.await`?
  - How to validate it?
    - Use single thread runtime? 