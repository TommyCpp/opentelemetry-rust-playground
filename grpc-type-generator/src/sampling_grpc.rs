// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_SAMPLING_MANAGER_GET_SAMPLING_STRATEGY: ::grpcio::Method<super::sampling::SamplingStrategyParameters, super::sampling::SamplingStrategyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/jaeger.api_v2.SamplingManager/GetSamplingStrategy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SamplingManagerClient {
    client: ::grpcio::Client,
}

impl SamplingManagerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SamplingManagerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_sampling_strategy_opt(&self, req: &super::sampling::SamplingStrategyParameters, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::sampling::SamplingStrategyResponse> {
        self.client.unary_call(&METHOD_SAMPLING_MANAGER_GET_SAMPLING_STRATEGY, req, opt)
    }

    pub fn get_sampling_strategy(&self, req: &super::sampling::SamplingStrategyParameters) -> ::grpcio::Result<super::sampling::SamplingStrategyResponse> {
        self.get_sampling_strategy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_sampling_strategy_async_opt(&self, req: &super::sampling::SamplingStrategyParameters, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::sampling::SamplingStrategyResponse>> {
        self.client.unary_call_async(&METHOD_SAMPLING_MANAGER_GET_SAMPLING_STRATEGY, req, opt)
    }

    pub fn get_sampling_strategy_async(&self, req: &super::sampling::SamplingStrategyParameters) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::sampling::SamplingStrategyResponse>> {
        self.get_sampling_strategy_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SamplingManager {
    fn get_sampling_strategy(&mut self, ctx: ::grpcio::RpcContext, req: super::sampling::SamplingStrategyParameters, sink: ::grpcio::UnarySink<super::sampling::SamplingStrategyResponse>);
}

pub fn create_sampling_manager<S: SamplingManager + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SAMPLING_MANAGER_GET_SAMPLING_STRATEGY, move |ctx, req, resp| {
        instance.get_sampling_strategy(ctx, req, resp)
    });
    builder.build()
}
