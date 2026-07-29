// let starts with store lists values with vectors:
// first type of collection is vector, what are the Vectors Vec<T>?
// Vectors allows us to store multiple values in One data structure,
// and it allows you to store a variable number of values next to each other in memory.

// vectors are very usefull when you have list of elements like list of products prices inside a cart.
fn main() {
// lets create new vector 
let mut ve : Vec<i32> = Vec::new();
// create empty vector can carry vector of i32 values.

// create a vector using macro 
let mut v = vec![1,2,3,4,5,6]; // here the compiler will understand that this vector will be i32

// update on avector:
ve.push(5);
ve.push(6);
ve.push(7);
ve.push(8);

// read vector values:
// there is two way to read the values of vectors:
//   1. By Indexing.
//   2. Using "get" Method. 

let third_vec_val: &i32= &v[2];// access by index
// let third_vec_val_using_get = ve.get(2); //it will show this error error
// error[E0277]: `Option<&i32>` doesn't implement `std::fmt::Display`
//   --> src/main.rs:29:72
//    |
// 29 | ...v using get method!: {third_vec_val_using_get}");
//    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ `Option<&i32>` cannot be formatted with the default formatter
   // so with get Method we must return Option Values
   let third_vec_val_using_get:Option<&i32> = ve.get(2);

    println!("this is the third element in vector v by indexing!: {third_vec_val}");
 
    match third_vec_val_using_get{
    Some(i)=>println!("this is the third element in vector v using get method!: {i}"),
    None=> println!("there is No Third Value")
}

    println!("---------------------------------------------------------");
    println!("--------------------------------------------");
    println!("---------------------------------------------------------");


// how RUST work when the index doesnt exist:
// let doesnt_exist=&v[99];// it will cause a Panic because it access on non exists element and the program will stop
// let doesnt_exist=&v.get(99); // here it will return a None becaues its a Option Struct.


    println!("-------------Ownership Rules and Borrow Checker with Vectors---------------------------");
// Ownership Rules and Borrow Checker with Vectors
let first=&v[0];
v.push(7);
// println!("this is the First element in vector v by indexing!: {first}");
// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
//   --> src/main.rs:56:1
//    |
// 55 | let first=&v[0];
//    |            - immutable borrow occurs here
// 56 | v.push(7);
//    | ^^^^^^^^^ mutable borrow occurs here
// 57 |     println!("this is the First element in vector v by indexing!: {first}");
//    |                                                                    ----- immutable borrow later used here

// Rust will stop the program, Why??
// 1. Because Rust put the elements of Vector next to each other.
// 2. When we add new element to the end of Vector this is need "Reallocating New Memory"
//    and move all old elements to the new Location and new size if the old location is not big enough for new size of vector
// 3. now if Rust Allow this operation the reference "&" will be points on Deallocated Memory, because we moved vector to another location.


    println!("-------------Iterating Over the Values in a Vector---------------------------");
//  Iterating Over the Values in a Vector
// we can access to all elements in a vector using for loop
for i in &v {
    println!("element {i}"); // i here not the value of element but it is the reference of current value it s lie &i 
}

for i in &mut v{
    *i += 50;
    println!("this is the new value of vector {i}");
    
}
// to change the value that the reference refers to, we have to use the"*" dereference opearator
// to get a the value of i before we can use += operator 
// more explanaition:
// let mut v = vec![100, 32, 57];
// v
// +-----+-----+-----+
// |100  | 32  | 57  |
// +-----+-----+-----+
// why we arnt write the i += 50; instead of *i += 50;?
// because i isnt a number its a reference like say Reference+50;
// So, we have to Dereference to change the value of element usimg *.
// https://chatgpt.com/share/6a6a10fd-1b64-83eb-8ad9-33611bcb7dc8
let firstt=&v[0];
println!("{firstt}");


    println!("---------------------------------------------------");
    println!("------------- Using an Enum to Store Multiple Types ---------------------------");
// Using an Enum to Store Multiple Types
// we know that Vectors works only with one type, But what happends if we need to 
// multiple types?
// Using Enums we can define the element types as a variant in Enum. for Example:
// assume that we have to collect values of the first row in a SpreadSheet. this row comtains different types of values.
// like integers, floats, and Strings. We have to identify a Enum contains the types of Columns\
enum SpreafSheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![
    SpreafSheetCell::Int(3).
    SpreafSheetCell::Float(5.6),
    SpreafSheetCell::Text(String::from("Pass")),
];

// Why Rust Have Know all types of vector before compile? because it include Copmile time
// to Know how this vector how memory size in heap needed to store each elemnt in vector.



// Dropping a Vector Drops Its Elements
// like any struct or value in rust, the vector will dropped its elements when Goes Out of scope
 }// <- v , ve, amd row goes out of scope and is freed here
// for more information please follow this link: https://share.gemini.google/NFORZglpoKnT