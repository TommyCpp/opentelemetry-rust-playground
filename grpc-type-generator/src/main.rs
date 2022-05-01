use protobuf_codegen::Customize;
use protoc_grpcio::compile_grpc_protos;
use std::error::Error;
use std::io;

mod sampling_json;

fn main() -> Result<(), io::Error> {
    // prost_build::compile_protos(&["src/sampling.proto"], &["src/"]).unwrap();
    // compile_grpc_protos(
    //     &[
    //         "src/sampling.proto"
    //     ],
    //     &["src/"],
    //     "src/",
    //     Some(Customize {
    //         expose_fields: Some(true),
    //         serde_derive: Some(true),
    //         ..Default::default()
    //     }),
    // )
    //     .expect("Error generating protobuf");
    test();

    Ok(())
}

fn test() {
    let x = r#"
    {"strategyType":"PROBABILISTIC","probabilisticSampling":{"samplingRate":0.8},"operationSampling":{"defaultSamplingProbability":0.8,"defaultLowerBoundTracesPerSecond":0,"perOperationStrategies":[{"operation":"op1","probabilisticSampling":{"samplingRate":0.2}},{"operation":"op2","probabilisticSampling":{"samplingRate":0.4}},{"operation":"/health","probabilisticSampling":{"samplingRate":0}},{"operation":"/metrics","probabilisticSampling":{"samplingRate":0}}],"defaultUpperBoundTracesPerSecond":0}}
    "#;
    let test: sampling_json::SamplingStrategyResponse = serde_json::from_str(x).unwrap();
    print!("{}", test.strategy_type == sampling_json::SamplingStrategyType::Probabilistic)
}
