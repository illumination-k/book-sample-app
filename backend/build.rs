use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    // tonic_build::compile_protos("../proto/book.proto");
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("book_descriptor.bin"))
        .compile(&["../proto-def/book.proto"], &["../proto-def"])?;
    Ok(())
}
