fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/");
    
    tonic_build::compile_protos("proto/blazer.proto")?;
    tonic_build::compile_protos("proto/message.proto")?;
    
    Ok(())
}