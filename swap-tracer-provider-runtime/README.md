# How to swap the tracer provider in runtime.

## Overview
This examples shows how to swap the tracer provider during runtime and shutdown the previous provider.
The use case of this includes:
- Prevent span collection(although users should consider Sampler first)
- Shut down span export

