# Double Faults
To prevent Tiple fautls ( CPU  fails to invoke an exception hanlder )

## set up an Interrupt Stack Table 
to catch double faults on a separate kernel stack.

### So it’s kind of similar to catch-all blocks in programming languages with exceptions, e.g., 
catch(...)

vextor number is : 8

## HANLDE MOST PAGE FAULT CASES WITH: 
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

## BUT THERE ARE SOME CASES EX: WHEN A FAULT HANLDER ITSELF CAUSES AN EXCEPTION
First Exception

### First Exception	Second Exception
Divide-by-zero,
Invalid TSS,
Segment Not Present,
Stack-Segment Fault,
General Protection Fault

### Second Exception
Invalid TSS,
Segment Not Present,
Stack-Segment Fault,
General Protection Fault

# STACK OVERFLOW
## A guard page is a special memory page at the bottom of a stack that makes it possible to detect stack overflows.

When a page fault occurs, the CPU looks up the page fault handler in the IDT and tries to push the interrupt stack frame onto the stack. However, the current stack pointer still points to the non-present guard page. Thus, a second page fault occurs, which causes a double fault (according to the above table).

So the CPU tries to call the double fault handler now. However, on a double fault, the CPU tries to push the exception stack frame, too. The stack pointer still points to the guard page, so a third page fault occurs, which causes a triple fault and a system reboot. So our current double fault handler can’t avoid a triple fault in this case.
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    […] // test_main(), println(…), and loop {}
}

# STACK OVERFLOW SOLUTOIN:: Switch Stacks
We can’t omit the pushing of the exception stack frame, since the CPU itself does it.we need to ensure somehow that the stack is always valid when a double fault exception occurs.
the x86_64 architecture has a solution

## Switching Stacks
The x86_64 architecture is able to switch to a predefined, known-good stack when an exception occurs. This switch happens at 
### hardware level, 
so it can be performed before the CPU pushes the exception stack frame.

The switching mechanism is implemented as an Interrupt Stack Table (IST). The IST is a table of 
### 7 pointers to known-good stacks. In Rust-like pseudocode:

struct InterruptStackTable {
    stack_pointers: [Option<StackPointer>; 7],
}

# TO LOAD THE TSS NEED: 
## GDT GLOBAL DESCRIPTOR TABLE and Paging 
### both isolate programs from each other

Loading the TSS
Now that we’ve created a new TSS, we need a way to tell the CPU that it should use it. Unfortunately, this is a bit cumbersome since the TSS uses the segmentation system (for historical reasons). Instead of loading the table directly, we need to add a new segment descriptor to the Global Descriptor Table (GDT). Then we can load our TSS by invoking the ltr instruction with the respective GDT index. (This is the reason why we named our module gdt.)

# GDT vs Paging
The Global Descriptor Table (GDT) is a relic that was used for memory segmentation before paging became the de facto standard. However, it is still needed in 64-bit mode for various things, such as kernel/user mode configuration or TSS loading.
