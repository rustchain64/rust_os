# Static Variables

The only way to modify a static variable is to encapsulate it in a Mutex type, which ensures that only a single &mut reference exists at any point in time.

Static variables always live for the complete runtime of the program, so there is no way to reclaim and reuse their memory when they’re no longer needed. Also, they have unclear ownership semantics and are accessible from all functions, 
so they need to be protected by a Mutex when we want to modify them.

# Dynamic Memory
stored on the heap ( allocate and deallocate )

 memory can be reused after it is freed ( deallocate )

# Common Errors ( Rust uses Ownership checked at compile time) No Garbage Scans
Apart from memory leaks, which are unfortunate but don’t make the program vulnerable to attackers, there are two common types of bugs with more severe consequences:

When we accidentally continue to use a variable after calling deallocate on it, we have a so-called use-after-free vulnerability. Such a bug causes undefined behavior and can often be exploited by attackers to execute arbitrary code.
When we accidentally free a variable twice, we have a double-free vulnerability. This is problematic because it might free a different allocation that was allocated in the same spot after the first deallocate call. Thus, it can lead to a use-after-free vulnerability again.
These types of vulnerabilities are commonly known, so one might expect that people have learned how to avoid them by now. But no, such vulnerabilities are still regularly found, for example this use-after-free vulnerability in Linux (2019), that allowed arbitrary code execution. A web search like use-after-free linux {current year} will probably always yield results. This shows that even the best programmers are not always able to correctly handle dynamic memory in complex projects.

## ownership that is able to check the correctness of dynamic memory operations at compile time.
##  this approach is that the programmer still has fine-grained control over the use of dynamic memory, just like with C or C++

# CALLERS OF ALLOCATE AND DEALLOCATE
# BOX DOES ALL THE WORK ( MOST IMPORTANT)
Allocations in Rust
Instead of letting the programmer manually call allocate and deallocate, the Rust standard library provides abstraction types that call these functions implicitly. The most important type is Box, which is an abstraction for a heap-allocated value. 

It provides a Box::new constructor function that takes a value, calls allocate with the size of the value, and then moves the value to the newly allocated slot on the heap. To free the heap memory again, the Box type implements the Drop trait to call deallocate when it goes out of scope:

{
    let z = Box::new([1,2,3]);
    […]
} // z goes out of scope and `deallocate` is called
This pattern has the strange name resource acquisition is initialization (or RAII for short). It originated in C++, where it is used to implement a similar abstraction type called std::unique_ptr.

# RAII ( resource acquisition is initialization )
This pattern has the strange name resource acquisition is initialization (or RAII for short). It originated in C++, where it is used to implement a similar abstraction type called std::unique_ptr.

let x = {
    let z = Box::new([1,2,3]);
    &z[1]
}; // z goes out of scope and `deallocate` is called
println!("{}", x);

## x is broken since z is no more.

error[E0597]: `z[_]` does not live long enough
 --> src/main.rs:4:9
  |
2 |     let x = {
  |         - borrow later stored here
3 |         let z = Box::new([1,2,3]);
4 |         &z[1]
  |         ^^^^^ borrowed value does not live long enough
5 |     }; // z goes out of scope and `deallocate` is called
  |     - `z[_]` dropped here while still borrowed
The terminology can be a bit confusing at first. Taking a reference to a value is called borrowing the value since it’s similar to a borrow in real life: You have temporary access to an object but need to return it sometime, and you must not destroy it. 
## By checking that all borrows end before an object is destroyed, the Rust compiler can guarantee that no use-after-free situation can occur.

### At Compile
memory safety
thread safety

# USE HEAP MEMOERY ONLY
dynamic lifetime or a variable size. 

## The most important type with a dynamic lifetime is Rc,
 which counts the references to its wrapped value and deallocates it after all references have gone out of scope. 
 ## Examples for types with a variable size are:
  Vec, 
  String,
  collection types

  that dynamically grow when more elements are added. 
  These types work by allocating a larger amount of memory when they become full, copying all elements over, and then deallocating the old allocation.

For our kernel, we will mostly need the collection types, for example, to store a list of active tasks when implementing multitasking in future posts.

# GLOBAL HEAP ALLOCATOR
## src/lib
extern crate alloc;
Contrary to normal dependencies, we don’t need to modify the Cargo.toml. The reason is that the alloc crate ships with the Rust compiler as part of the standard library, so the compiler already knows about the crate. By adding this extern crate statement, we specify that the compiler should try to include it.extern crate alloc;
Contrary to normal dependencies, we don’t need to modify the Cargo.toml. The reason is that the alloc crate ships with the Rust compiler as part of the standard library, so the compiler already knows about the crate. By adding this extern crate statement, we specify that the compiler should try to include it.

## GlobalAlloc
### note allocate/deallocate used in Box::new() etx

#### unsafe, unsafe, unsafe
pub unsafe trait GlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { ... }
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize
    ) -> *mut u8 { ... }
}

# raw pointer, ( fine grained control) not safe
vs
# smart pointer ( are data structures ) safer

## The trait additionally defines the two methods alloc_zeroed and realloc with default implementations:

The alloc_zeroed method is equivalent to calling alloc and then setting the allocated memory block to zero, which is exactly what the provided default implementation does. An allocator implementation can override the default implementations with a more efficient custom implementation if possible.

The realloc method allows to grow or shrink an allocation. The default implementation allocates a new memory block with the desired size and copies over all the content from the previous allocation. Again, an allocator implementation can probably provide a more efficient implementation of this method, for example by growing/shrinking the allocation in-place if possible.

# Unsafety
One thing to notice is that both the trait itself and all trait methods are declared as unsafe:

The reason for declaring the trait as unsafe is that the programmer must guarantee that the trait implementation for an allocator type is correct. For example, the alloc method must never return a memory block that is already used somewhere else because this would cause undefined behavior.
Similarly, the reason that the methods are unsafe is that the caller must ensure various invariants when calling the methods, for example, that the Layout passed to alloc specifies a non-zero size. This is not really relevant in practice since the methods are normally called directly by the compiler, which ensures that the requirements are met.

# MEMORY ALLOCATION
We now have a simple allocator, but we still have to tell the Rust compiler that it should use this allocator. This is where the #[global_allocator] attribute comes in.

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

# Creating a Kernel Heap
use structures::paging::{ mapper....FrameAllocator}

Mat to phyxical frame
or page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

# IMPORTANT DATA STRUCTS BEYOND BOX, VEC, RC

Of course, there are many more allocation and collection types in the alloc crate that we can now all use in our kernel, including:

the thread-safe reference counted pointer
    Arc
    String  ( owned string type  )
    format! (macro)
    LinkedList
    VecDeque ( the growable ring buffer )
    BinaryHeap ( priority QUEUE)
    BTreeMap
    BTreeSet