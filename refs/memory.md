# Memory Protection
Isolate Programs

# MPU ( Memory Protection Unit )
Example, some ARM Cortex-M processors (used for embedded systems) have a Memory Protection Unit (MPU),

Defines Memory Regeions

# with different access permissions 
e.g., 
no access, 
read-only, 
read-write

## MPU ensures that the address is in a region with correct access permissions and throws an exception otherwise

Process Switching

## Two approaches to memmory protection
1) Segmentation ( The segment register is chosen automatically by the CPU (default))

The segment register is chosen automatically by the CPU depending on the kind of memory access: For fetching instructions, the code segment CS is used, and for stack operations (push/pop), the stack segment SS is used. Other instructions use the data segment DS or the extra segment ES. Later, two additional segment registers, FS and GS, were added, which can be used freely.

By loading separate global/local descriptor tables for each process, which confine memory accesses to the process’s own memory areas, the OS can isolate processes from each other.

etching instructions, the code segment CS is used

## By modifying the memory addresses before the actual access, segmentation already employed a technique that is now used almost everywhere: virtual memory.

2) Viertual Memory

Translation Functions

# PAGE TABLES

We saw that each of the potentially millions of pages is individually mapped to a frame. This mapping information needs to be stored somewhere. Segmentation uses an individual segment selector register for each active memory region, which is not possible for paging since there are way more pages than registers. Instead, paging uses a table structure called page table to store the mapping information.

For our above example, the page tables would look like this:

Three page tables, one for each program instance. For instance 1, the mapping is 0->100, 50->150, 100->200. For instance 2, it is 0->300, 50->350, 100->400. For instance 3, it is 0->250, 50->450, 100->500.

We see that each program instance has its own page table. A pointer to the currently active table is stored in a special CPU register. On x86, this register is called CR3. It is the job of the operating system to load this register with the pointer to the correct page table before running each program instance.

On each memory access, the CPU reads the table pointer from the register and looks up the mapped frame for the accessed page in the table. This is entirely done in hardware and completely invisible to the running program. To speed up the translation process, many CPU architectures have a special cache that remembers the results of the last translations.

Depending on the architecture, page table entries can also store attributes such as access permissions in a flags field. In the above example, the “r/w” flag makes the page both readable and writable.

# PAGE TABLE FORMAT

#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

We see that only bits 12–51 are used to store the physical frame address. The remaining bits are used as flags or can be freely used by the operating system. This is possible because we always point to a 4096-byte aligned address, either to a page-aligned page table or to the start of a mapped frame. This means that bits 0–11 are always zero, so there is no reason to store these bits because the hardware can just set them to zero before using the address. The same is true for bits 52–63, because the x86_64 architecture only supports 52-bit physical addresses (similar to how it only supports 48-bit virtual addresses).

# The Translation Lookaside Buffer
A 4-level page table makes the translation of virtual addresses expensive because each translation requires four memory accesses. To improve performance, the x86_64 architecture caches the last few translations in the so-called translation lookaside buffer (TLB). This allows skipping the translation when it is still cached.

# PAGE FAULTS

virtual address that caused the page fault. We use the Cr2::read function of the x86_64 crate to read and print it. The PageFaultErrorCode type provides more information about the type of memory access that caused the page fault, for example, whether it was caused by a read or write operation. For this reason, we print it too. We can’t continue execution without resolving the page fault, so we enter a hlt_loop at the end.

## CAUSE A PAGE FAULT
Now we can try to access some memory outside our kernel:

into main
// new
    let ptr = 0xdeadbeaf as *mut u8;
    unsafe { *ptr = 42; }

# Summary
This post introduced two memory protection techniques: segmentation and paging. While the former uses variable-sized memory regions and suffers from external fragmentation, the latter uses fixed-sized pages and allows much more fine-grained control over access permissions.

Paging stores the mapping information for pages in page tables with one or more levels. The x86_64 architecture uses 4-level page tables and a page size of 4 KiB. The hardware automatically walks the page tables and caches the resulting translations in the translation lookaside buffer (TLB). This buffer is not updated transparently and needs to be flushed manually on page table changes.

We learned that our kernel already runs on top of paging and that illegal memory accesses cause page fault exceptions. We tried to access the currently active page tables, but we weren’t able to do it because the CR3 register stores a physical address that we can’t access directly from our kernel.

# Using OffsetPageTable
Translating virtual to physical addresses is a common task in an OS kernel, therefore the x86_64 crate provides an abstraction for it. The implementation already supports huge pages and several other page table functions apart from translate_addr, so we will use it in the following instead of adding huge page support to our own implementation.

At the basis of the abstraction are two traits that define various page table mapping functions:
The 
Mapper trait is generic over the page size and provides functions that operate on pages. 

Examples are translate_page, 
which translates a given page to a frame of the same size, 

and map_to, which creates a new mapping in the page table.
The Translate trait provides functions that work with multiple page sizes, such as 
translate_addr 
or the general translate.

# The traits only define the interface, they don’t provide any implementation. 
The x86_64 crate currently provides three types that implement the traits with different requirements. The 

## OffsetPageTable 
type assumes that the complete physical memory is mapped to the virtual address space at some offset. 
## MappedPageTable 
is a bit more flexible: It only requires that each page table frame is mapped to the virtual address space at a calculable address. Finally, the 
## RecursivePageTable 
type can be used to access page table frames through recursive page tables.

# With the boot info frame allocator, the mapping succeeds and we see the black-on-white “New!” on the screen again. Behind the scenes, the map_to method creates the missing page tables in the following way:

Use the passed frame_allocator to allocate an unused frame.
Zero the frame to create a new, empty page table.
Map the entry of the higher level table to that frame.
Continue with the next table level.