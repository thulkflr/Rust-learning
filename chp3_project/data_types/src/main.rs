use std::io;

fn main() {
        
        let integer_variable: i32 = 10; //i32 is a 32-bit signed integer type.
        let float_variable: f64 = 10.5; //f64 is a 64-bit floating point type.
        let unsigned_sized_integer_variable: usize = 20; //usize is an unsigned integer type that is used for indexing and measuring the size of collections. It is typically the same size as the pointer size of the target architecture (e.g., 64 bits on a 64-bit system).
        let signed_sized_integer_variable: isize = 10; //isize is a signed integer type that is used for indexing and measuring the size of collections. It is typically the same size as the pointer size of the target architecture (e.g., 64 bits on a 64-bit system).
        //usize and isize are dependent on the architecture of the system, if we are on a 64-bit system, usize and isize will be 64 bits, if we are on a 32-bit system, usize and isize will be 32 bits. They are used for indexing and measuring the size of collections because they can represent the maximum size of a collection on the target architecture.
        
        let boolean_variable: bool = true; //bool is a boolean type, it can be either true or false.
        let character_variable: char = 'A'; //char is a character type, it can be any unicode character.
        let string_variable: String = String::from("Hello, World!"); //String is a string type, it can be any string of characters.
        let array_variable: [i32; 5] = [1, 2, 3, 4, 5]; //array is a fixed-size collection of elements of the same type. In this case, it is an array of 5 i32 integers.
        let first_element_of_array: i32 = array_variable[0]; //accessing the first element of the array. In this case, it is the first element of the array_variable, which is 1.
        let last_element_of_array: i32 = array_variable[array_variable.len() - 1]; //accessing the last element of the array. In this case, it is the last element of the array_variable, which is 5. We use the len() method to get the length of the array, and then we subtract 1 to get the index of the last element.
     
        
        let tuple_variable: (i32, f64, char) = (10, 10.5, 'A'); //tuple is a fixed-size collection of elements of different types. In this case, it is a tuple of an i32 integer, an f64 floating
        let (x, y, z) = tuple_variable; //destructuring a tuple into individual variables. In this case, x is an i32 integer, y is an f64 floating point number, and z is a char character.
        
        println!("integer_variable: {unsigned_sized_integer_variable}");



//Invalid array index handling example or Invalid array access handling example:
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num)=>num,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };
    
    let element = a[index]; //if the user enters an index that is out of bounds, this will panic and terminate the program. In a real application, you would want to handle this more gracefully, perhaps by checking if the index is within bounds before trying to access the array.
    // this is how rust's memory safety works, 
    //it prevents you from accessing memory that you don't own, which can lead to undefined behavior and security vulnerabilities. 
    //In this case, if the user enters an index that is out of bounds, rust will panic and terminate the program, preventing any potential memory safety issues.  
  
    println!("The value of the element at index {index} is: {element}");



    }
