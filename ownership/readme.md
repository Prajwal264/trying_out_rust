## Ownership is a set of rules that governs rust.

There are 3 ways of managing memory.
1. In some programming languages. They have a dedicated Garbage collection to free consumed memory.
2. In some programming languages. The programmer would have to allocate and free comsumed memory.

Rust takes a third approach where, memory is mangaed via ownership.


### Stack and Heap
Stack is used for storing the data that has a fixed size. 
Data with an unknown or varying size is store in heap.

The heap is less organized: when you put data on the heap, you request a certain amount of space. 
The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer,

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; 
that location is always at the top of the stack.
Comparatively, allocating space on the heap requires more work 
because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

#### Rules of ownership
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

A scope is the range within a program for which an item is valid (same as js)

#### Ownership and functions

