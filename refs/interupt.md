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