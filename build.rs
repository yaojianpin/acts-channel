fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature= "build_proto")]
    tonic_build::configure()
        .out_dir("proto")
        .compile(&["acts.proto"], &["proto"])?;
    Ok(())
}
