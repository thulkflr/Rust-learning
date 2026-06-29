//Ownership is a set of rules that govern how a Rust program manages memory.
// The ownership system is designed to ensure memory safety without needing a garbage collector.
// The ownership system is based on three main principles:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

//Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks. 
//If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

//The Stack and the Heap
//in a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.
//Parts of ownership will be described in relation to the stack and the heap later in this chapter, so here is a brief explanation in preparation.
//1. The stack is a place for data that is stored in a last-in, first-out (LIFO) manner.
//adding called pushing and removing called popping. The stack has a fixed size, and the size of the data must be known at compile time.
//All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.


//2. The heap is a place for data that is allocated in an arbitrary order and whose size might not be known at compile time.
//Heap is less organized than the stack, and it is slower to access data in the heap than in the stack. 
//When you put data on the heap, you request a certain amount of space. The operating system finds an empty spot in the heap that is big enough, 
//marks it as being in use, and returns a pointer to that location. This process is called allocating on the heap. 
//and is sometimes abbreviated(يختصر ب) as just allocating (pushing values onto the stack is not considered allocating).
//Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.


//Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data;
//that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work
//because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

//Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there.



fn main() {

//Variable Scope
//Variables are valid only from the point at which they are declared to the end of the block in which they are declared.
//The scope of a variable is the range within a program for which that variable is valid.
//When a variable goes out of scope, Rust calls a special function for that variable. This function is called drop, and it’s where the author of the type can put the code to clean up the value.
//The drop function is called automatically at the closing curly bracket of the scope in which the variable is valid.
//example of variable scope
    let x = 5; //x is valid from this point forward
    println!("The value of x is: {x}");
    {
        let y = 10; //y is valid from this point forward
        println!("The value of y is: {y}");
    } //y goes out of scope here
    //println!("The value of y is: {y}"); //this line will cause an error because y is no longer valid
    println!("The value of x is: {x}"); //x is still valid here 
    

    //The String Type
    //The String type is a growable, heap-allocated data structure that is used to store text.
    //The String type is provided by Rust’s standard library, and it is the most commonly used string type in Rust. The String type is a wrapper around a vector of bytes, and it
    //provides a number of methods for manipulating strings. The String type is also UTF-8 encoded, which means that it can store any valid Unicode character.
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `Hello, world!` 
    
    //explore how heap allocation and deallocation works with the String type
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2, and s1 is no longer valid. This is because the String type is not a Copy type, and it is moved instead of copied. The ownership of the String is transferred from s1 to s2,
    //and s1 is no longer valid. This is because the String type is not a Copy type, and it is moved instead of copied. The ownership of the String is transferred from s1 to s2, and s1 is no longer valid. This is because the String type is not a Copy type, and it is moved instead of copied. The ownership of the String is transferred from s1 to s2, and s1 is no longer valid. This is because the String type is not a Copy type, and it is moved instead of copied. The ownership of the String is transferred from s1 to s2, and s1 is no longer valid. This is because        
    

    // difference between string literals and String type
    //String literals are stored in the binary of the program, and they are immutable.
    //The String type is stored on the heap, and it is mutable. 
    //The String type is a growable, heap-allocated data structure that is used to store text. The String type is provided by Rust
    //example of string literals and String type
    let s3 = "Hello"; // s3 is a string literal, and it is immutable
    let mut s4 = String::from("Hello"); // s4 is a String type  

}
