# To build an image post build

rustup component add llvm-tools-preview

To turn our compiled kernel into a bootable disk image, we need to link it with a bootloader. As we learned in the section about booting, the bootloader is responsible for initializing the CPU and loading our kernel.

Instead of writing our own bootloader, which is a project on its own, we use the bootloader crate. This crate implements a basic BIOS bootloader without any C dependencies, just Rust and inline assembly. To use it for booting our kernel, we need to add a dependency on it:

# in Cargo.toml

[dependencies]
bootloader = "0.9"

Note: This post is only compatible with bootloader v0.9. Newer versions use a different build system and will result in build errors when following this post.

Adding the bootloader as a dependency is not enough to actually create a bootable disk image. The problem is that we need to link our kernel with the bootloader after compilation, but cargo has no support for post-build scripts.

To solve this problem, we created a tool named bootimage that first compiles the kernel and bootloader, and then links them together to create a bootable disk image. To install the tool, go into your home directory (or any directory outside of your cargo project) and execute the following command in your terminal:

# cargo install bootimage
For running bootimage and building the bootloader, you need to have the llvm-tools-preview rustup component installed. You can do so by executing 

rustup component add llvm-tools-preview.

After installing bootimage and adding the llvm-tools-preview component, you can create a bootable disk image by going back into your cargo project directory and executing:

## run 
> cargo bootimage

### comiled boot image
Created bootimage for `rust_os` at `/home/wparr/Projects/rust_mentor/embedded/rust_os/rust_os/target/x86_64-rust_os/debug/bootimage-rust_os.bin`

# Target Table
## The target.'cfg(target_os = "none")' table 
applies to all targets whose target configuration file’s "os" field is set to "none". This includes our x86_64-blog_os.json target. The runner key specifies the command that should be invoked for cargo run. The command is run after a successful build with the executable path passed as the first argument. See the cargo documentation for more details.