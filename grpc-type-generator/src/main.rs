use std::error::Error;
use std::io;
use protobuf_codegen::Customize;
use protoc_grpcio::compile_grpc_protos;

fn main() -> Result<(), io::Error> {
    compile_grpc_protos(
        &[
            "src/sampling.proto"
        ],
        &["src/"],
        "src/",
        Some(Customize {
            expose_fields: Some(true),
            serde_derive: Some(true),
            ..Default::default()
        }),
    )
        .expect("Error generating protobuf");

    Ok(())
}
