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
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize {// s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.
 