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

//////////////////////////////////////////////
//https://claude.ai/share/5d2c1884-2608-480b-9a52-ca4966a53c70 link to the claude.ai chat that helped me understand ownership and memory management in Rust.
//////////////////////////////////////////////


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
    let s4 = String::from("Hello"); // s4 is a String type  
    // s4 here points to a heap-allocated string, its stored in a stack.




//Memory and Allocation
//Memory is a limited resource, and it is important to manage it properly. Rust provides a
//memory management system that ensures safety and efficiency.
//we dont have in Rust a garbage collector,  
//it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
//Rust takes a different path: The memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
           
     // this scope is now over, and s is no
    // longer valid
    // Rust automatically calls the drop function for s, and the memory is freed. This is because the String type is not a Copy type,
    // and it is moved instead of copied. The ownership of the String is transferred from s to the drop function, and s is no longer valid. 
    // when varialble like s goes out of scope, Rust automatically calls the drop function for s,and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket
//This pattern has a profound impact on the way Rust code is written. It may seem simple right now, 
//but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we’ve allocated on the heap. Let’s explore some of those situations now.
//هاد النمط (تحرير الـ memory عند نهاية الـ curly bracket) بسيط جداً بس بيفتح باب لمشاكل لو تعاملنا معه بسذاجة، وهون بيجي دور الـ Move.

//Variables and Data Interacting with Move
// Multiple variables can interact with the same data in different ways in Rust. Listing 4-2 shows an example using an integer.
    let x = 5;
    let y = x;
// Listing 4-2: Assigning the integer value of variable x to y
// We can probably guess what this is doing: “Bind the value 5 to x; then, make a copy of the value in x and bind it to y.” We now have two variables, x and y, and both equal 5. This is indeed what is happening, because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

//now let’s look at a similar example using a String, which is a more complex type that we’ve already seen is stored on the heap. Listing 4-3 shows an example of this.
    let s6 = String::from("hello");
    let s7 = s6;
// Listing 4-3: Assigning the String value of variable s6 to s7
// This code is similar to the previous example, but it behaves differently. The String type is stored on the heap, and the variable s6 is a pointer to that heap data. When we assign s6 to s7, we’re not copying the heap data. Instead, we’re copying
//the pointer, which means that both s6 and s7 point to the same heap data. This is called a move, and it’s a key concept in Rust’s ownership system. After the move, s6 is no longer valid, and we can’t use it anymore. If we try to use s6 after the move, we’ll get a compile-time error.
//مؤشر (pointer) بيشاور على الـ memory اللي فيها محتوى النص، وهاي محفوظة على الـ heap.
// الطول (len).
// السعة (capacity).

// لما نكتب let s7 = s6;، Rust بتنسخ الـ pointer والـ len والـ capacity بس (يعني اللي عالـ stack)، وما بتنسخ محتوى النص اللي عالـ heap. النتيجة: صار عندنا s6 و s7 وكلاهون بيشاورو على نفس المكان بالـ heap.
// هون بتظهر مشكلة كبيرة: لما s6 و s7 يطلعو برا الـ scope، كل وحدة رح تحاول تعمل drop لنفس الـ memory. هاد اسمه double free error، وهو bug خطير بيسبب data corruption أو ثغرات أمنية    println!("{s6}, world!");
    println!("{s6}, world!");

    //it will show an error because s6 is no longer valid after the move. The ownership of the String has been transferred to s7, 
    //the responsibility of freeing the memory is now on s7, and s6 is no longer valid. If we try to use s6 after the move, we’ll get a compile-time error. This is because Rust’s ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.
    //Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2. So, what actually happens
} 



//1.2 Scope and Assignment
//When a variable is assigned to another variable, the ownership of the value is transferred to the new variable. The original variable is no longer valid, and any attempt to use it will result in a compile-time error. This is known as a move. The ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
//The ownership of the String value is transferred from the first String to the second String, and the first String is no longer valid. The memory for the first String is automatically freed when it goes out of scope, and the memory for the second String is automatically freed when it goes out of scope. This ensures that there are no double free errors and that memory is managed safely and efficiently.


//Variables and Data Interacting with Clone
// The clone method can be used to create a deep copy of a value, which means that the heap data is also copied. This allows both variables to own their own copy of the data, and both variables are valid after the clone. However, cloning can be expensive in terms of performance, especially for large data structures, so it should be used judiciously.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    // this is deep copy, because the heap data is also copied, and both variables are valid after the clone. However,
    //cloning can be expensive in terms of performance, especially for large data structures, so it should be used judiciously. 


//Ownership and Functions
//When a variable is passed to a function, the ownership of the value is transferred to the function. The original variable is no longer valid, and any attempt to use it will result in a compile-time error. This is known as a move. The ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    println!("{s}, world!"); //this line will cause an error because s is no longer valid after the move. The ownership of the String has been transferred to the function, and the responsibility of freeing the memory is now on the function. If we try to use s after the move, we’ll get a compile-time error. This is because Rust’s ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.


    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward  
    println!("{x}, world!"); //this line will not cause an error because x is still valid after the copy. The ownership of the i32 value is copied to the function, and the original variable x is still valid. This is because i32 is a Copy type, and it is copied instead of moved. The ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.




//Return Values and Scope
//When a function returns a value, the ownership of the value is transferred to the caller. The original variable in the function is no longer valid, and any attempt to use it will result in a compile-time error. This is known as a move. The ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    println!("{s1}, world!"); //this line will not cause an error because s 
    //is still valid after the move. The ownership of the String value is transferred from the function to the variable s1, and the original variable in the function is no longer valid. This is because Rust’s ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.
    //The gives_ownership function returns a String value, and the ownership of that value is transferred to the variable s1. The original variable in the function is no longer valid, and any attempt to use it will result in a compile-time error. This is known as a move. The ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.
    //The ownership system ensures that there is only one owner of a value at a time, and when the owner goes out of scope, the value will be dropped. This prevents double free errors and ensures memory safety.
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also
                                        // moves its return value into s3
    println!("{s3}, world!"); //this line will not cause an error because s3 is still valid after the move. The ownership of the String value is transferred from the variable s2 to the function takes_and_gives_back, and then back to the variable s3. The

}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
//If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. These static checks protect us from mistakes. Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling function
}   
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}