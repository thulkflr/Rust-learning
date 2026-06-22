fn main() {
   // Functions in Rust:
   // Functions are a way to organize code into reusable blocks. They allow you to break down your code into smaller, more manageable pieces, and they can be called from other parts of your code
   //BASIC SYNTAX OF A FUNCTION:
   // fn function_name(parameters) -> return_type {
   //     // function body
   // }
   // - function_name: the name of the function, it should be in snake_case.
   // - parameters: the input values that the function takes, they are optional, and they should be in the format of parameter_name: parameter_type. You can have multiple parameters separated by commas.
   // - return_type: the type of value that the function returns, it is optional, and it should be in the format of -> return_type. If the function does not return a value, you can omit the return type or use () as the return type.
   // - function body: the code that defines what the function does, it is enclosed in curly braces {}. 
    testFunction();
    testFunctionWithParams(10);



    // Expressions and Statements in Rust:
    // In Rust, an expression is a piece of code that evaluates to a value, while
    // a statement is a piece of code that performs an action but does not return a value.
    // For example, the following is an expression:
    let x = 5 + 10; //5 + 10 is an expression that evaluates to 15, and the value is assigned to the variable x.
    // The following is a statement:
    println!("The value of x is: {x}"); //println! is a macro that prints the value of x to the console, but it does not return a value
    //, so it is a statement.

    // Expressions can be used in statements, but statements cannot be used in expressions. For example, the following is valid:
    let y = {
        let z = 5 + 10; //5 + 10 is an expression that evaluates to 15, and the value is assigned to the variable z. This is a statement.
        z + 5 //z + 5 is an expression that evaluates to 20, and the value is returned from the block. This is an expression.
    };
    println!("The value of y is: {y}"); //This will print "The value of y is: 20" to the console.   


    // Functions with parameters and return values:
    let sum = addTwoNumbers(5, 10); //this will call the function addTwoNumbers with the arguments 5 and 10, and the return value will be assigned to the variable sum.
    println!("function with return type: The sum of 5 and 10 is: {sum}"); //This will print "The sum of 5 and 10 is: 15" to the console.

}


fn testFunction(){
    println!("this is a test function");
}


fn testFunctionWithParams(number: i32){
    println!("this is the Number for tedtFunctionWithParams: {number}");
}

// functuions with return values:
fn addTwoNumbers(a: i32, b: i32) -> i32 {//this function must return an i32 value, because we specified the return type as i32. If we do not return a value, or if we return a value of a different type, we will get a compile-time error.

   // a+b;
    //this error shows error[E0308]: mismatched types
//   --> src/main.rs:51:37
//    |
// 51 | fn addTwoNumbers(a: i32, b: i32) -> i32 {//this function must return an i32 value, because we specified the return type as i32. If ...
//    |    -------------                    ^^^ expected `i32`, found `()`
//    |    |
//    |    implicitly returns `()` as its body has no tail or `return` expression
// 52 |     a + b; //this is an expression that evaluates to the sum of a and b, and the value is returned from the function. We do not nee...
//    |          - help: remove this semicolon to return this value
    
    //To fix this error, we need to remove the semicolon at the end of the expression a + b, so that it becomes the last expression in the function and is automatically returned.
    a + b //this is an expression that evaluates to the sum of a and b, and the value is returned from the function. We do not need to use the return keyword because the last expression in the function is automatically returned.
    // or we can use the return keyword to explicitly return a value from the function, like this:
    // return a + b; //this will also return the sum of a and b, but it is not necessary to use the return keyword in this case because the last expression in the function is automatically returned.
}