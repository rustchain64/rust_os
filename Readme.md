# Refernece
os.phil-opp.com/freestanding-rust-binary/

# To build without error have no underlying os
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf

# custom target
## Linux
cargo rustc -- -C link-arg=-nostartfiles
## Mac OS
cargo rustc -- -C link-arg="-e __start -static -nostartfiles"