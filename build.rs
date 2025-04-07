use std::env;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let pb_env = env::var("GENERATE_PB").unwrap_or_else(|_| "0".to_string());
    if pb_env == "1" {
        generate_pb()?;
    } else {
        // println!("cargo:warning=Skipping protobuf generation");
    }

    Ok(())
}

fn generate_pb() -> Result<()> {
    let output_dir = PathBuf::from("src/generated/pb");

    prost_build::Config::new()
        .out_dir(&output_dir)
        .default_package_filename("a2o")
        .compile_protos(&["pb/A2o.proto"], &["pb"])?;

    prost_build::Config::new()
        .out_dir(&output_dir)
        .default_package_filename("o2a")
        .compile_protos(&["pb/O2a.proto"], &["pb"])?;

    println!(
        "cargo:warning=Generated protobuf files in {}",
        output_dir.display()
    );

    Ok(())
}
