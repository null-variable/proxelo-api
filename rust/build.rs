use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../proto");
    let files = [
        root.join("proxelo/snapshot/v1/snapshot.proto"),
        root.join("proxelo/distrib/v1/distrib.proto"),
    ];
    let fds = protox::compile(&files, [&root])?;
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .build_transport(false)
        .compile_fds(fds)?;
    println!("cargo:rerun-if-changed={}", root.display());
    Ok(())
}
