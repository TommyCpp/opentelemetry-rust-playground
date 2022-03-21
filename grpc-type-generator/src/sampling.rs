/// ProbabilisticSamplingStrategy samples traces with a fixed probability.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbabilisticSamplingStrategy {
    /// samplingRate is the sampling probability in the range [0.0, 1.0].
    #[prost(double, tag = "1")]
    pub sampling_rate: f64,
}
/// RateLimitingSamplingStrategy samples a fixed number of traces per time interval.
/// The typical implementations use the leaky bucket algorithm.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitingSamplingStrategy {
    /// TODO this field type should be changed to double, to support rates like 1 per minute.
    #[prost(int32, tag = "1")]
    pub max_traces_per_second: i32,
}
/// OperationSamplingStrategy is a sampling strategy for a given operation
/// (aka endpoint, span name). Only probabilistic sampling is currently supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationSamplingStrategy {
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub probabilistic_sampling: ::core::option::Option<ProbabilisticSamplingStrategy>,
}
/// PerOperationSamplingStrategies is a combination of strategies for different endpoints
/// as well as some service-wide defaults. It is particularly useful for services whose
/// endpoints receive vastly different traffic, so that any single rate of sampling would
/// result in either too much data for some endpoints or almost no data for other endpoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerOperationSamplingStrategies {
    /// defaultSamplingProbability is the sampling probability for spans that do not match
    /// any of the perOperationStrategies.
    #[prost(double, tag = "1")]
    pub default_sampling_probability: f64,
    /// defaultLowerBoundTracesPerSecond defines a lower-bound rate limit used to ensure that
    /// there is some minimal amount of traces sampled for an endpoint that might otherwise
    /// be never sampled via probabilistic strategies. The limit is local to a service instance,
    /// so if a service is deployed with many (N) instances, the effective minimum rate of sampling
    /// will be N times higher. This setting applies to ALL operations, whether or not they match
    /// one of the perOperationStrategies.
    #[prost(double, tag = "2")]
    pub default_lower_bound_traces_per_second: f64,
    /// perOperationStrategies describes sampling strategiesf for individual operations within
    /// a given service.
    #[prost(message, repeated, tag = "3")]
    pub per_operation_strategies: ::prost::alloc::vec::Vec<OperationSamplingStrategy>,
    /// defaultUpperBoundTracesPerSecond defines an upper bound rate limit.
    /// However, almost no Jaeger SDKs support this parameter.
    #[prost(double, tag = "4")]
    pub default_upper_bound_traces_per_second: f64,
}
/// SamplingStrategyResponse contains an overall sampling strategy for a given service.
/// This type should be treated as a union where only one of the strategy field is present.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamplingStrategyResponse {
    /// Legacy field that was meant to indicate which one of the strategy fields
    /// below is present. This enum was not extended when per-operation strategy
    /// was introduced, because extending enum has backwards compatiblity issues.
    /// The recommended approach for consumers is to ignore this field and instead
    /// checks the other fields being not null (starting with operationSampling).
    /// For producers, it is recommended to set this field correctly for probabilistic
    /// and rate-limiting strategies, but if per-operation strategy is returned,
    /// the enum can be set to 0 (probabilistic).
    #[prost(enumeration = "SamplingStrategyType", tag = "1")]
    pub strategy_type: i32,
    #[prost(message, optional, tag = "2")]
    pub probabilistic_sampling: ::core::option::Option<ProbabilisticSamplingStrategy>,
    #[prost(message, optional, tag = "3")]
    pub rate_limiting_sampling: ::core::option::Option<RateLimitingSamplingStrategy>,
    #[prost(message, optional, tag = "4")]
    pub operation_sampling: ::core::option::Option<PerOperationSamplingStrategies>,
}
/// SamplingStrategyParameters defines request parameters for remote sampler.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamplingStrategyParameters {
    /// serviceName is a required argument.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
}
/// See description of the SamplingStrategyResponse.strategyType field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SamplingStrategyType {
    Probabilistic = 0,
    RateLimiting = 1,
}
#[doc = r" Generated client implementations."]
pub mod sampling_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " SamplingManager defines service for the remote sampler."]
    #[derive(Debug, Clone)]
    pub struct SamplingManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SamplingManagerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SamplingManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SamplingManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SamplingManagerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get_sampling_strategy(
            &mut self,
            request: impl tonic::IntoRequest<super::SamplingStrategyParameters>,
        ) -> Result<tonic::Response<super::SamplingStrategyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/jaeger.api_v2.SamplingManager/GetSamplingStrategy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod sampling_manager_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SamplingManagerServer."]
    #[async_trait]
    pub trait SamplingManager: Send + Sync + 'static {
        async fn get_sampling_strategy(
            &self,
            request: tonic::Request<super::SamplingStrategyParameters>,
        ) -> Result<tonic::Response<super::SamplingStrategyResponse>, tonic::Status>;
    }
    #[doc = " SamplingManager defines service for the remote sampler."]
    #[derive(Debug)]
    pub struct SamplingManagerServer<T: SamplingManager> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SamplingManager> SamplingManagerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SamplingManagerServer<T>
    where
        T: SamplingManager,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/jaeger.api_v2.SamplingManager/GetSamplingStrategy" => {
                    #[allow(non_camel_case_types)]
                    struct GetSamplingStrategySvc<T: SamplingManager>(pub Arc<T>);
                    impl<T: SamplingManager>
                        tonic::server::UnaryService<super::SamplingStrategyParameters>
                        for GetSamplingStrategySvc<T>
                    {
                        type Response = super::SamplingStrategyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SamplingStrategyParameters>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_sampling_strategy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSamplingStrategySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: SamplingManager> Clone for SamplingManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SamplingManager> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SamplingManager> tonic::transport::NamedService for SamplingManagerServer<T> {
        const NAME: &'static str = "jaeger.api_v2.SamplingManager";
    }
}
