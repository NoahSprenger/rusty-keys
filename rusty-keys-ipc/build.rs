use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/ipc_messages.proto"], &["src/"])?;
    Ok(())
}