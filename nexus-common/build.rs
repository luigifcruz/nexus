use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::compile_protos("proto/generic.proto")?;
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptors_generic.bin"))
        .compile(&["proto/generic.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/enums.proto")?;
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptors_enums.bin"))
        .compile(&["proto/enums.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/meta.proto")?;
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptors_meta.bin"))
        .compile(&["proto/meta.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/replicant.proto")?;
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptors_replicant.bin"))
        .compile(&["proto/replicant.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/instance.proto")?;
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptors_instance.bin"))
        .compile(&["proto/instance.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/nexus.proto")?;
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("descriptors_nexus.bin"))
        .compile(&["proto/nexus.proto"], &["proto"])?;

    Ok(())
}
