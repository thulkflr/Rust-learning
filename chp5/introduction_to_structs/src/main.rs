// Structs
// Structs are used to create custom data types in Rust. 
// They allow you to group related data together and define your own types with named fields.
// In this example, we will define a struct called `Person` that has two fields: `name` and `age`.
struct Person {
    name: String,
    age: u8,
}

struct User{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
//to use this struct we can create an instance of it and access its fields using dot notation.


//Creating Different Types with Tuple Structs
// Tuple structs are similar to regular structs, but they don't have named fields. Instead, they have a tuple of values.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Defining Unit-Like Structs
// Unit-like structs are similar to regular structs, but they don't have any fields. They are useful when you want to implement a trait 
//on some type but don't need to store any data in the struct.
struct AlwaysEqual;

fn main() {
// now we gonna create an instance of the `Person` struct and access its fields 
let user1 =User{
    username: String::from("user1"),
    email: String::from("user@gmail.user"),
    sign_in_count: 1,
    active: true,   
};




if false&&false{
    let user3=User{
        username: String::from("user3"),
        ..user1};//here we move the values of the fields from user1 to user3, except for the fields that we explicitly set.
//so if we use user1 after this line, we will get an error because user1 has been moved to user3.
}

//The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance

// we can access the fields of the struct using dot notation
dbg!(&user1.username);
dbg!(&user1.email);
dbg!(&user1.sign_in_count);
dbg!(&user1.active);//it will displey error because user1 has been moved to user3, so we can't access its fields anymore.


if false&&false{
    dbg!(&user3.username);
    dbg!(&user3.email);
    dbg!(&user3.sign_in_count);
    dbg!(&user3.active);
}

let userByFunction= build_first_user(&user1.email, &user1.username);
dbg!(&userByFunction.username);
dbg!(&userByFunction.email);


let black=Color(0,0,0);
let origin=Point(0,0,0);
dbg!(&black.0);
dbg!(&origin.0);

let subject=AlwaysEqual;// we can create an instance of the unit-like struct using the same syntax as a regular struct, but without any fields.

}

fn build_first_user(email:&str, username:&str)->User{
    User{
        email:email.to_string(),// this is called field init shorthand, it allows us to use the same name for the field and the variable that we are passing in
        username:username.to_string(),// we are using the to_string method to convert the &String to a String, because the field email is of type String and we are passing in a &String
        sign_in_count:1,
        active:true,
    }
}
