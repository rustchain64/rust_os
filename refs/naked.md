# https://os.phil-opp.com/edition-1/extra/naked-exceptions/
# https://os.phil-opp.com/handling-exceptions/   x86-interrupt

## Handling Exceptions using naked Functions
These posts explain how to handle CPU exceptions using naked functions. Historically, these posts were the main exception handling posts before the x86-interrupt calling convention and the x86_64 crate existed. Our new way of handling exceptions can be found in the “Handling Exceptions” post.

### CATCHING EXCEPTIONS
We set up an interrupt descriptor table and add handler functions. At the end of this post, our kernel will be able to catch divide-by-zero faults.

As always, the complete source code is on GitHub. Please file issues for any problems, questions, or improvement suggestions. There is also a comment section at the end of this page.

Note: This post describes how to handle exceptions using naked functions (see “Handling Exceptions with Naked Functions” for an overview). Our new way of handling exceptions can be found in the “Handling Exceptions” post.

# NAKED INLINE ASSEMBLY
Inline Assembly
In order to cause a divide-by-zero exception, we need to execute a div or idiv assembly instruction with operand 0. We could write a small assembly function and call it from our Rust code. An easier way is to use Rust’s inline assembly macro.

Inline assembly allows us to write raw x86 assembly within a Rust function. The feature is unstable, so we need to add #![feature(asm)] to our src/lib.rs. Then we’re able to write a divide_by_zero function:

fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}

Let’s try to decode it:

### The asm! macro emits raw assembly instructions, so it’s unsafe to use it.
We insert two assembly instructions here: mov dx, 0 and div dx. The former loads a 0 into the dx register (a subset of rdx) and the latter divides the ax register by dx. (The div instruction always implicitly operates on the ax register).

The colons are separators. After the first : we could specify output operands and after the second : we could specify input operands. We need neither, so we leave these areas empty.
After the third colon, we specify the so-called clobbers. These tell the compiler that our assembly modifies the values of some registers. Otherwise, the compiler assumes that the registers preserve their value. In our case, we clobber dx (we load 0 to it) and ax (the div instruction places the result in it).

The last block (after the 4th colon) specifies some options. The volatile option tells the compiler: “This code has side effects. Do not delete it and do not move it elsewhere”. In our case, the “side effect” is the divide-by-zero exception. Finally, the intel option allows us to use the Intel assembly syntax instead of the default AT&T syntax.

### Use it
// in src/lib.rs

pub extern "C" fn rust_main(...) {
    ...

    // provoke a divide-by-zero fault
    divide_by_zero();

    println!("It did not crash!");
    loop {}
}