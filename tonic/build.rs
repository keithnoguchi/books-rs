//! [tonic] [build] [hello] world example
//!
//! [tonic]: https://lib.rs/tonic
//! [build]: https://github.com/hyperium/tonic/blob/master/tonic-build/README.md
//! [hello]: https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(tonic_build::configure()
        .build_client(false)
        .out_dir("./src/")
        .compile(&["proto/hello.proto"], &["proto"])?)
}
