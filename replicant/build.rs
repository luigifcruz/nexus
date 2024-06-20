use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("../proto/generic.proto")?;
    tonic_build::compile_protos("../proto/enums.proto")?;

    tonic_build::compile_protos("../proto/nexus.proto")?;
    tonic_build::compile_protos("../proto/replicant.proto")?;

    Ok(())
}