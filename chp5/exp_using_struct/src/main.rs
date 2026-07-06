//rectuangle with struct

//we can use a struct to represent a rectangle and its dimensions.
//we use derive Debug to be able to print the struct using the {:?} specifier in the println! macro.
//#[derive(Debug)] explains to the compiler that we want to use the Debug trait for the struct. 

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {

    let rect_area = area_of_rect(10, 5);
    let testRectArea = return_rect_area(rect_area);
    let tuple_area = area_with_tuple((10, 5));
    let rectStruct = Rectangle{
        height: 10,
        width: 5,
    };
    println!("The area of the rectangle is {}", rect_area);
    println!("The area of the rectangle is {}", testRectArea);
    println!("The area of the rectangle is {}", tuple_area);
    println!("The area of the rectangle is {}", area_with_struct(&rectStruct));
    println!("The area of the rectangle is {}", rectStruct.height*rectStruct.width);

  //println!("The area of the rectangle is {}", rectStruct);//this will display error because we can't print a struct directly, 
    //                                     --   ^^^^^^^^^^ `Rectangle` cannot be formatted with the default formatter
    //    |                                 |
    //    |                                 required by this formatting parameter
    //we need to implement the `Debug` trait for the struct to be able to print it.
    //to implement the `Debug` trait for the struct, we can use the `#[derive(Debug)]` attribute on the struct definition.
    // with struts when we use macro println! 
    //should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? 
    //Should all the fields be shown? Due to this ambiguity,
    //Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of Display to use with println! and the {} placeholder    

//lets try with Putting the specifier :?
    //println!("The area of the rectangle is {:?}", rectStruct);//this will display error because we can't print a struct directly, 
    //                                        --   ^^^^^^^^^^ `Rectangle` cannot be formatted with the default formatter
    //    |                                   |
    //    |                                  required by this formatting parameter
    //we need to implement the `Debug` trait for the struct to be able to print it.
    //to implement the `Debug` trait for the struct, we can use the `#[derive(Debug)]` attribute on the struct definition.

    // lets try after using derive Debug above the struct definition
    println!("The rectangle is {:?}", rectStruct);
    dbg!(&rectStruct);//this will display the struct in a more readable format, with the field names and values. because
    //we used the `Debug` trait for the struct, we can use the `dbg!` macro to print the struct in a more readable format.



    let rect3 = Rectangle{
        height: dbg!(30*6),
        width: 3,
    };
    dbg!(&rect3);

}


fn area_of_rect(length:u32, width:u32)->u32{
    length*width
}

fn return_rect_area(rect:u32)->u32{
    rect
}

fn area_with_tuple(dimns:(u32,u32))->u32{
    dimns.0*dimns.1
    //dimns.0 and dimns.1 are the first and second elements of the tuple, respectively.
}
fn area_with_struct(rect:&Rectangle)->u32{
    rect.height*rect.width
}