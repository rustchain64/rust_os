# CPU Exceptions
set up an interrupt descriptor table that provides 
handler functions

handler()

## On x86, there are about 20 different CPU exception type

## Most important are listed below
Page fault: illiegal memmory access

Invalid Opcode: SSE instructions on old CPU that does not support.

General Protection Fault: Broadest range of causes
Access violation:
priveleged instruction execution in user level code
writing reserved fields in config registers

Double Fault: exception occurs
CPU tries to call the corersponding function handler function
If whild calling the exeption hanlder 
CPU raises Double Fault...

Tiple Fault: exception occurs
if while double fault, another exception is thrown, it issues a Triple fault.
no Handle exception, it proccessor will "Re boot"
See OSDev Wiki for more information


# The Interrupt Descriptor Table from x86_64 crate
## https://docs.rs/x86_64/latest/x86_64/structures/idt/struct.InterruptDescriptorTable.html

#[repr(C, align(16))]
pub struct InterruptDescriptorTable (be sure to expand)
[-]


#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_by_zero: Entry<HandlerFunc>,
    pub debug: Entry<HandlerFunc>,
    pub non_maskable_interrupt: Entry<HandlerFunc>,
    pub breakpoint: Entry<HandlerFunc>,
    pub overflow: Entry<HandlerFunc>,
    pub bound_range_exceeded: Entry<HandlerFunc>,
    pub invalid_opcode: Entry<HandlerFunc>,
    pub device_not_available: Entry<HandlerFunc>,
    pub double_fault: Entry<HandlerFuncWithErrCode>,
    pub invalid_tss: Entry<HandlerFuncWithErrCode>,
    pub segment_not_present: Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
    pub page_fault: Entry<PageFaultHandlerFunc>,
    pub x87_floating_point: Entry<HandlerFunc>,
    pub alignment_check: Entry<HandlerFuncWithErrCode>,
    pub machine_check: Entry<HandlerFunc>,
    pub simd_floating_point: Entry<HandlerFunc>,
    pub virtualization: Entry<HandlerFunc>,
    pub security_exception: Entry<HandlerFuncWithErrCode>,
    // some fields omitted
}
                <------ Original Pointer
Return Address          8 bytes
                <------ New Stack Ponter
Stack Frame of the Handler Function
the function itself has the : error code

interupt stack frame:

cs Means code Segment

x86-interupt calling convention
there is a post handling naked functions

# START WITH BREAKPOINT EXCEPTON ( as in a debugger)
start by adding a handler for the breakpoint exception. 
The breakpoint exception is the perfect exception to test exception handling. 
Its only purpose is to temporarily pause a program when the 
breakpoint instruction int3 is executed.

When the user sets a breakpoint, the debugger overwrites the corresponding instruction with the int3 instruction so that the CPU throws the breakpoint exception when it reaches that line. When the user wants to continue the program, the debugger replaces the int3 instruction with the original instruction again and continues the program. For more details, see the “How debuggers work” series.

## The x86-interrupt calling convention and the InterruptDescriptorTable

# int3 function
#[test_case]
fn test_breakpoint_exeption() {
     // >>>> BREAKPOINT exception ... this is for test not actual breakpoint in main
     x86_64::instructions::interrupts::int3();
}

# TOO MUCH MAGIC
## GO NAKED
see naked.md

### ////////////////////////////////////////////////////////////////////////////////
# PIC 5259  pic new ( start ) --> APIC ( upgrade )
Intel 5259 Progammable interrupt controller (PIC)

Connecting all hardware devices directly to the CPU is not possible. Instead, a separate interrupt controller aggregates the interrupts from all devices and then notifies the CPU:

## HARDWARE INTERUPTS OCCUR ASYNCRONOUSLY 
### INTERUPT CONTORLLERS
Priorities:
Timers
Keypoard

# Conrrency Related Bugs.
Dead Locks

ownership model helps us here because it forbids mutable global state

This graphic shows the typical assignment of interrupt lines. We see that most of the 15 lines have a fixed mapping, e.g., line 4 of the secondary PIC is assigned to the mouse.

Each controller can be configured through two I/O ports, one “command” port and one “data” port. For the primary controller, these ports are 0x20 (command) and 0x21 (data). For the secondary controller, they are 0xa0 (command) and 0xa1 (data). For more information on how the PICs can be configured, see the article on osdev.org.

# Due to Overlap: Must remap the PIC interrupts to different numbers
The default configuration of the PICs is not usable because it sends interrupt vector numbers in the range of 0–15 to the CPU. These numbers are already occupied by CPU exceptions. For example, number 8 corresponds to a double fault.

# AFTER EXCEPTION SLOTS
not overlap with the exceptions, but typically the range of 32–47 is chosen


# Configure the Timer PIT
https://wiki.osdev.org/Programmable_Interval_Timer

# Deadlocks

We now have a form of concurrency in our kernel: The timer interrupts occur asynchronously, so they can interrupt our _start function at any time. Fortunately, Rust’s ownership system prevents many types of concurrency-related bugs at compile time. One notable exception is deadlocks. Deadlocks occur if a thread tries to acquire a lock that will never become free. Thus, the thread hangs indefinitely.

We can already provoke a deadlock in our kernel. Remember, our println macro calls the vga_buffer::_print function, which locks a global WRITER using a spinlock:

// in src/vga_buffer.rs

[…]

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

The WRITER is locked, so the interrupt handler waits until it becomes free. But this never happens, because the _start function only continues to run after the interrupt handler returns. Thus, the entire system hangs.

# RACE CONDITION

> cargo test --lib
[…]
Running 4 tests
test_breakpoint_exception...[ok]
test_println... [ok]
test_println_many... [ok]
test_println_output... [failed]

Error: panicked at 'assertion failed: `(left == right)`
  left: `'.'`,
 right: `'S'`', src/vga_buffer.rs:205:9

 ## Race condition between test and timer handler. The test looks like this:

// in src/vga_buffer.rs

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

# HLT hault put CPU to sleep in betweern interrupts

halt the CPU until the next interrupt arrives. This allows the CPU to enter a sleep state in which it consumes much less energy.


# Keyboard Input 
Only prints one "K" unless we read the Scancodes

To find out which key was pressed, we need to query the keyboard controller. We do this by reading from the data port of the PS/2 controller, which is the I/O port with the number 0x60:

# scan code sets
there are three scan code sets. 
ps/2 keyboards use scancode set one.

## Interpreting the Scancodes
There are three different standards for the mapping between scancodes and keys, the so-called scancode sets. All three go back to the keyboards of early IBM computers: the IBM XT, the IBM 3270 PC, and the IBM AT. Later computers fortunately did not continue the trend of defining new scancode sets, but rather emulated the existing sets and extended them. Today, most keyboards can be configured to emulate any of the three sets.

## By default, PS/2 keyboards emulate scancode set 1 (“XT”). 
In this set, the lower 7 bits of a scancode byte define the key, and the most significant bit defines whether it’s a press (“0”) or a release (“1”). Keys that were not present on the original IBM XT keyboard, such as the enter key on the keypad, generate two scancodes in succession: a 0xe0 escape byte and then a byte representing the key. For a list of all set 1 scancodes and their corresponding keys, check out the OSDev Wiki.

let key = match scancode {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0a => Some('9'),
        0x0b => Some('0'),
        _ => None,
    };
    if let Some(key) = key {
        print!("{}", key);
    }

 By using the same variable name key in the pattern, we shadow the previous declaration, which is a common pattern for destructuring Option types in Rust.

 extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
We use the lazy_static macro to create a static Keyboard object protected by a Mutex. We initialize the Keyboard with a US keyboard layout and the scancode set 1. The HandleControl parameter allows to map ctrl+[a-z] to the Unicode characters U+0001 through U+001A. We don’t want to do that, so we use the Ignore option to handle the ctrl like normal keys.

On each interrupt, we lock the Mutex, read the scancode from the keyboard controller, and pass it to the add_byte method, which translates the scancode into an Option<KeyEvent>. The KeyEvent contains the key which caused the event and whether it was a press or release event.

To interpret this key event, we pass it to the process_keyevent method, which translates the key event to a character, if possible. For example, it translates a press event of the A key to either a lowercase a character or an uppercase A character, depending on whether the shift key was pressed.

With this modified interrupt handler, we can now write text:

# Configuring the Keyboard
It’s possible to configure some aspects of a PS/2 keyboard, for example, which scancode set it should use. We won’t cover it here because this post is already long enough, but the OSDev Wiki has an overview of possible configuration commands.
https://wiki.osdev.org/PS/2_Keyboard#Commands

# Summary
This post explained how to enable and handle external interrupts. We learned about the 8259 PIC and its primary/secondary layout, the remapping of the interrupt numbers, and the “end of interrupt” signal. We implemented handlers for the hardware timer and the keyboard and learned about the hlt instruction, which halts the CPU until the next interrupt.

Now we are able to interact with our kernel and have some fundamental building blocks for creating a small shell or simple games.

# LET THE KERNEL REGAIN CONTROL
What’s next?
Timer interrupts are essential for an operating system because they provide a way to periodically interrupt the running process and let the kernel regain control. The kernel can then switch to a different process and create the illusion of multiple processes running in parallel.

# HOW TO CREATE THREADS
But before we can create processes or threads, we need a way to allocate memory for them. The next posts will explore memory management to provide this fundamental building block.