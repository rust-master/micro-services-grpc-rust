// build.rs act as build script for cargo, in we configure the tonic build

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/payment.proto")?;
    Ok(())
}