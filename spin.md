# 'spin::Mutex'
Adding spin to Your Cargo.toml
First, you need to include the spin crate in your project by adding it to your Cargo.toml:

toml
Copy code
[dependencies]
spin = "0.9" # Check for the latest version on crates.io
Using spin::Mutex
Here's a simple example of how to use spin::Mutex:

rust
Copy code
extern crate spin;

use spin::Mutex;

// Global mutex. Statics require explicitly stating the type.
static MY_MUTEX: Mutex<i32> = Mutex::new(0);

fn main() {
    // Lock the mutex and access the data
    let mut data = MY_MUTEX.lock();
    *data += 1;
    println!("Data: {}", *data);
    // Mutex is automatically unlocked at the end of the scope
}
Characteristics and Use Cases
No Blocking: spin::Mutex does not block the thread waiting for the lock to become available. Instead, it keeps trying to acquire the lock in a loop. This is beneficial in environments where blocking is not possible or desired (like in interrupt handlers or very low-level system programming).

OS-independent: It's suitable for environments without an OS, such as bare-metal programming in embedded systems or writing an OS kernel.

Overhead and CPU Usage: Because it continually polls for the lock, it can consume more CPU resources, especially if the lock is held for a significant amount of time or the contention is high.

Use in Interrupts: It's safe to use in interrupt context within certain conditions, unlike many blocking mutexes.

In the context of embedded Rust, particularly when using platforms like the STM32F3 Discovery board, the spin::Mutex can be crucial for protecting shared resources (like hardware registers) from concurrent access in interrupt routines or in a multi-threaded context where no operating system is available to provide more complex synchronization primitives.