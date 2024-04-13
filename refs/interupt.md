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