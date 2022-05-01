# What happens if we export spans in random order to Jaeger?

## Setup
1. Start a jaeger collector/backend:
```
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest
```
2. Set up a jaeger exporter: Build a parent span and a child span. Send the child span first and then send the parent span.

## Conclusion
As suspected, Jaeger can handle out of order span ingestion. 
