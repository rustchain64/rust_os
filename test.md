# There is an issue with running tests not returning therefor there is an endless loop at the end of our start function.

## QEMU supports a special isa-debug-exit device, which provides an easy way to exit QEMU from the guest system

# Exit QEMU from the guest
[package.metadata.bootimage]
test-args = ["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]
The bootimage runner appends the test-args to the default QEMU command for all test executables. For a normal cargo run, the arguments are ignored.

Together with the device name (isa-debug-exit), we pass the two parameters iobase and iosize that specify the I/O port through which the device can be reached from our kernel.

# PORTS 0xf4 is a general purpose device.
Port-mapped I/O uses a separate I/O bus for communication. Each connected peripheral has one or more port numbers. To communicate with such an I/O port, there are special CPU instructions called in and out, which take a port number and a data byte (there are also variations of these commands that allow sending a u16 or u32).

## The isa-debug-exit device uses port-mapped I/O. 
The iobase parameter specifies on which port address the device should live (0xf4 is a generally unused port on the x86’s IO bus) and the iosize specifies the port size (0x04 means four bytes).

# Using the Exit Device
The functionality of the isa-debug-exit device is very simple. When a value is written to the I/O port specified by iobase, it causes QEMU to exit with exit status (value << 1) | 1. So when we write 0 to the port, QEMU will exit with exit status (0 << 1) | 1 = 1, and when we write 1 to the port, it will exit with exit status (1 << 1) | 1 = 3.

Instead of manually invoking the in and out assembly instructions, we use the abstractions provided by the x86_64 crate. To add a dependency on that crate, we add it to the dependencies section in our Cargo.toml:

## src/main
#[repr(u32)]

Hex code 0x10 is 16
Hex code 0x11 is 17
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

# The chips implementing a serial interface are called UARTs. There are lots of UART models on x86, but fortunately the only differences between them are some advanced features we don’t need. The common UARTs today are all compatible with the 16550 UART, so we will use that model for our testing framework.

We will use the uart_16550 crate to initialize the UART and send data over the serial port. To add it as a dependency, we update our Cargo.toml and main.rs:

# Print to consolde

