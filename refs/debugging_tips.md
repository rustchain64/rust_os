# print the underlying heap pointers 
using the {:p} format specifier

let heap_value = Box::new(41);
println!("heap_value at {:p}", heap_value);

# DEBUG: ADDRESS OF A POINTER
# In Rust, the {:p} format specifier used within the println! macro is designed to print the memory address of a pointer. When you see {:p} in a formatting string, it means the argument provided should be a pointer, and the output will be the address to which the pointer points, formatted as a hexadecimal value.

# let heap_value = Box::new(41);
## println!("heap_value at {:p}", heap_value);
heap_value is a Box containing an integer (41). Box is a smart pointer that allocates memory on the heap and takes ownership of the value.
When you use 
### {:p} in the println! statement with heap_value as the argument, 
it prints the memory address of the 41 stored on the heap.

The output will show the address in hexadecimal format, which points to where the 41 is located in memory.
Important Notes
This usage is safe and common for debugging purposes where understanding memory layout and pointer usage is necessary.
It's also worth noting that you're not directly printing the Box itself but the address where its contained data (41) is located, thanks to the smart pointer's inherent dereferencing capabilities.
The use of {:p} can be particularly useful when diagnosing issues related to data location, memory leaks, or pointer arithmetic in more complex systems where raw pointers (*const T, *mut T) are used.
Overall, the {:p} specifier is a straightforward and useful tool in Rust for handling and displaying pointer addresses in a human-readable format.