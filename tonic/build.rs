//! [tonic] [build] [hello] world and [route] guide example
//!
//! [tonic]: https://lib.rs/tonic
//! [build]: https://github.com/hyperium/tonic/blob/master/tonic-build/README.md
//! [hello]: https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
//! [route]: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(tonic_build::configure()
        .out_dir("./src/")
        .compile(&["proto/hello.proto", "proto/route.proto"], &["proto"])?)
}
