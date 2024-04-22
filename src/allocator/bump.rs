use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

/*
    We chose to create a separate init function instead of 
    performing the initialization directly in new in order to 
    keep the interface identical to the allocator provided by 
    the linked_list_allocator crate. 
    This way, the allocators can be switched without 
    additional code changes.
*/

impl BumpAllocator {
    /// Creates a new empty bump allocator.
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }
    // allocations is just a counter

    /// Initializes the bump allocator with the given heap bounds.
    ///
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused. Also, this method must be called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

/*
INTEREIR IMMUTABILITY
    As we saw in the previous post, the global heap allocator is defined 
    by adding the #[global_allocator] attribute to a static that implements 
    the GlobalAlloc trait. Static variables are immutable in Rust, 
    so there is no way to call a method that takes &mut self on the static allocator. 
    For this reason, all the methods of GlobalAlloc only take an immutable 
    &self reference.

    Fortunately, there is a way to get a &mut self reference from a &self reference: 
    We can use synchronized interior mutability by wrapping the allocator 
    in a spin::Mutex spinlock.
 */

// we have to own one of the sides in order for this to work
// either GlobalAlloc or spin::Mutex<>
// to get ownership we create a wrapper type around spin::Mutex
// TO permit trait implementation
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock(); // get a mutable reference

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            ptr::null_mut() // out of memory
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock(); // get a mutable reference

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}

