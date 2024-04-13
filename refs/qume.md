# QEMU
## set to this to the runner
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin

# REAL MACHINE
It is also possible to write it to a USB stick and boot it on a real machine, but be careful to choose the correct device name, because everything on that device is overwritten:

dd if=target/x86_64-rust_os/debug/bootimage-rust_os.bin of=/dev/sdX && sync

# Where sdX is the device name of your USB stick.

After writing the image to the USB stick, you can run it on real hardware by booting from it. You probably need to use a special boot menu or change the boot order in your BIOS configuration to boot from the USB stick. Note that it currently doesn’t work for UEFI machines, since the bootloader crate has no UEFI support yet.

# Create Runner for Cargo Run
To make it easier to run our kernel in QEMU, we can set the runner configuration key for cargo:

# in .cargo/config.toml

[target.'cfg(target_os = "none")']
runner = "bootimage runner"