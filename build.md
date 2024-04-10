# Comments for build
cargo build --target x86_64-rust_os.json

error[E0463]: can't find crate for `core`

## This tells cargo that it should recompile the core and compiler_builtins libraries. The latter is required because it is a dependency of core. In order to recompile these libraries, cargo needs access to the rust source code, which we can install with 

## installed by default

rustup component add rust-src

# Can't build without nightly 
## try this from the compiler
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

## versus rust-toolchain.json
cargo +nightly build --target x86_64-rust_os.json