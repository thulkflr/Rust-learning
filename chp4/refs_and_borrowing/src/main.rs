//References and Borrowing
//here we moved the string s1 into the function calculate_length, so we can no longer use s1 after that point
fn main() {

    // let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1);
    //     dbg!(&s1);// this will throw an error because s1 has been moved into the function calculate_length, so we can no longer use s1 after that point
    //     dbg!(&s2);// this will work because s2 is a new variable that has taken ownership of the value that was moved into the function calculate_length
    // println!("The length of '{s2}' is {len}.");


//instead of moving the string into the function, we can pass a reference to the string, which allows us to refer to some value without taking ownership of it
let s1 = String::from("hello");
let len = calculate_length_with_reference(&s1);
dbg!(&s1);// this will work because s1 is still valid after the function call, because we passed a reference to it instead of moving it into the function
dbg!(&len);// this will work because len is a new variable that has taken ownership of the value that was returned from the function calculate_length_with_reference
println!("The length of '{s1}' is {len}.");


//areferences is like a pointer, so the value it points to must be valid, we can follow the address in the reference to access the data, but we cannot modify the value through a reference
//The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because the reference does not own it, the value it points to will not be dropped when the reference stops being used.

//References are immutable by default, so we cannot modify the value through a reference. 
//If we want to modify the value through a reference, we can use a mutable reference, which is declared with the mut keyword.
//Borrowing is the act of creating a reference to a value without taking ownership of it.
//lets take a look at an example of borrowing with mutable references
let mut s = String::from("hello");
change(&mut s);// we are passing a mutable reference to the function change, which allows us to modify the value of s through the reference
dbg!(&s);// this will work because s is still valid after the function call, because we passed a mutable reference to it instead of moving it   

// tale alook at an example of borrowing with immutable references
if false{
    let s = String::from("hello");
    // change_with_immutable_reference(&s);// we are passing an immutable reference to the function change_with_immutable_reference, which does not allow us to modify the value of s through the reference
    dbg!(&s);// this will work because s is still valid after the function call, because we passed an immutable reference to it instead of moving it
}

// Mutable references have one big restriction: If you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
// This code does not compile!
        if false{
            let mut ss = String::from("hello");

            let r1 = &mut ss;
            let r2 = &mut ss; // error: cannot borrow `ss` as mutable more than once at a time
                // ^^^^^^^ second mutable borrow occurs here

            //this cause a data race, which is when two or more pointers access the same data at the same time. At least one of them is being used to write to the data, and the data is not protected by locks         
            // To fix this, we can create a new scope for the first mutable reference, so that it goes out of scope before we create the second mutable reference
        }


    let mut sss = String::from("hello");
        {  
        let rs1 = &mut sss;
    } // rs1 goes out of scope here, so we can make a new reference with no problems.
    let rs2 = &mut sss; // no problem

    // println!("{rs1}, {rs2}");// this will throw an error because rs1 has gone out of scope, so we can no longer use it after that point
    println!("{rs2}");// this will work because rs2 is still valid after the scope of rs1, because we created a new reference to sss after rs1 went out of scope



//Dangling References
    // Dangling references are references that point to data that has been dropped. 
    // Rust prevents dangling references by ensuring that references always point to valid data.
    // The following code will not compile because it attempts to return a reference to a value that will be dropped when the function ends:
    let reference_to_nothing = dangle();// this will throw an error because the function dangle returns a reference to a value that will be dropped when the function ends

    // To fix this, we can return the value itself instead of a reference to it. 
    // This way, the value will be moved out of the function and into the calling code, and it will not be dropped when the function ends.
    // The following code will compile because it returns the value itself instead of a reference to it:
    let s = no_dangle();// this will work because the function no_dangle returns the
    // value itself instead of a reference to it, so the value will be moved out of the function and into the calling code, and it will not be dropped when the function ends


//The Rules of References
// Let’s recap what we’ve discussed about references:

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

}
fn dangle() -> &String {// this function returns a reference to a String
    let s = String::from("hello");// s is a new String
    &s// we are returning a reference to s, but s will be dropped when the function ends, so this will cause a dangling reference
} // Here, s goes out of scope and is dropped. Its memory goes away. Danger! 
//but the reference that was returned from the function dangle is still pointing to the memory that was allocated for s,
//which has been dropped, so the reference is now dangling and points to invalid memory
//this is why Rust prevents dangling references by ensuring that references always point to valid data. 



fn no_dangle() -> String {// this function returns a String
    let s = String::from("hello");// s is a new String
    s// we are returning the value of s, so it will be moved out of the function and into the calling code, and it will not be dropped when the function ends
} // Here, s goes out of scope. But because it was returned, it is not dropped.


fn change(some_string: &mut String) {// some_string is a mutable reference to a String
    some_string.push_str(", world");// we can modify the value of s through the mutable reference some_string
} // Here, some_string goes out of scope. But because it does not have ownership of what
  // it refers to, the String is not dropped.   


// fn chenge_with_immutable_reference(some_string: &String) {// some_string is an immutable reference to a String
//     // some_string.push_str(", world");// this will throw an error because we cannot modify the value through an immutable reference
// } // Here, some_string goes out of scope. But because it does not have ownership of what
  // it refers to, the String is not dropped.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize {// s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

  
 