# Write to addresss
## Safe by encapsulating all unsafety in a separate module
Don’t access the RAM but directly access the text buffer on the VGA hardware with:

0xb8000

# Memory Layout
#[repr(transparent)]

in systems programming, you often need explicit control over the layout to ensure compatibility with other languages or hardware-specific memory layouts.

Transparent Representation: When you annotate a struct or enum with #
same as its single non-zero-sized field
Foreign Function Interface (FFI) Compatibility
Rust data structure can be used interchangeably with a corresponding C type
This is particularly useful when writing safe wrappers around C libraries.
## allows you to create safe abstractions over these hardware registers.
#[repr(transparent)]

## SafeInt is guaranteed to have the same memory layout as u32, so it can be safely passed to the C function c_function
#[repr(transparent)]
struct SafeInt(u32);

extern "C" {
    fn c_function(p: *const SafeInt);
}

fn main() {
    let s = SafeInt(10);
    unsafe {
        c_function(&s);
    }
}

## Zero Cost
Using #[repr(transparent)] is essential for ensuring compatibility and safety when interfacing Rust code with other languages or directly with hardware in embedded systems. It allows for the creation of abstractions that are both type-safe and zero-cost in terms of performance.

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

//The 'static lifetime specifies that the reference is valid for 
//the whole program run time (which is true for the VGA text buffer

# The VGA text buffer only supports ASCII Rust strings are UTF-8 by default
and the additional bytes of code page 437. Rust strings are UTF-8 by default, so they might contain bytes that are not supported by the VGA text buffer. We use a match to differentiate printable ASCII bytes (a newline or anything in between a space character and a ~ character) and unprintable bytes. For unprintable bytes, we print a ■ character, which has the hex code 0xfe on the VGA hardware.


# Volital implementations to future proof against compiler optimizations.
Volital has both read/write.
 self.buffer.chars[row][col].write(ScreenChar {

# Formatting Macros

# Static Writer
To understand what’s happening here, we need to know that 
## statics are initialized at compile time, 
in contrast to normal variables that are initialized at run time. 
The component of the Rust compiler that evaluates such initialization expressions is called the “const evaluator”. 
Its functionality is still limited, but there is ongoing work to expand it, for example in the “Allow panicking in constants” RFC.

## Use Lazy Static 
Rust’s const evaluator is not able to convert raw pointers to references at compile time.
 Lazy Statictic now lazily initializes itself when accessed for the first time, not at compile time.

 # Spin Locks
 there is a really basic kind of mutex in computer science that requires no operating system features: the 
 ## spinlock