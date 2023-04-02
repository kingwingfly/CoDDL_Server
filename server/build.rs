fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("../pb/src")
        .build_client(false)
        .compile(&["../proto/sign.proto"], &["../"])?;
    Ok(())
}
