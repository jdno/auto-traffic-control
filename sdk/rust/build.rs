use std::path::PathBuf;

use glob::glob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../api");

    let proto_path = PathBuf::from("../../api");

    let protocol_buffers: Vec<PathBuf> = glob("../../api/**/*.proto")
        .unwrap()
        .map(|path| path.unwrap())
        .collect();

    let build_server = std::env::var("CARGO_FEATURE_SERVER").is_ok();

    tonic_build::configure()
        .build_server(build_server)
        .compile(&protocol_buffers, &[proto_path.to_str().unwrap()])?;

    Ok(())
}
