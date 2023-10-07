fn main() {
    // There are two subsets of data types in rust
    // 1. Scalar
    // 2. Compound
    //
    // Scalar:=
    // Scalar types usually represent a single value.
    // eg: Integer, Float, Booleans, Characters
    //
    // Integer - is a number without any floating point
    // u32 - unsigned int 32bits - This means it allocates 32bits of space for a large positive
    // integer. i32 stands for signed 32bit interger. It makes sense to use u8 - if we know in
    // advance that the number would be a positive integer that would only need 8 bits to represent
    // itself.
    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive,
    // where n is the number of bits that variant uses.
    // So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127.
    // Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28
    // - 1, which equals 0 to 255.
    //
    // Float - is a number with floating point
    // f32 and f64 are the two types of float. Both are signed by default
    // f64 is used for double precision
    //
    // Boolean - true or false
    //
    // Chars - chars are usually 4 bit in memeory and are encoded in ''
    //
    // Compound:=
    // Compound types are a group of multiple values in one type
    // There are two primitive compound types = Tuples and Arrays
    //
    // Tuple - A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    //
    // Array - Another way to have a collection of multiple values is with an array.
    // Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
    // A Vector is like an array, but its allowed to grow or shrink in size
    // Use an Array when you are sure of the size, else use a vector
}
