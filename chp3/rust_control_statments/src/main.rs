use std::io;

fn main() {
    let condition: bool = true;
    let numberForTest= if condition { 9 } else { 10 };
    println!("The value of numberForTest is: {numberForTest}");

    let mut selected_option  =String::new();
    // conditiotions with loops
    let mut counter = 0;
    println!("Please select an option: 1 for loop, 2 for loop with labels , 3 for while loop , 4 for loop , 5 for loop with arrays");
    io::stdin().read_line(&mut selected_option).expect("Failed to read line");



    let result =if selected_option.trim()== "1" { loop {
        counter += 1 ;
        if counter%2==0{
            println!("The counter is even: {counter}");
        } if counter==1000000{
            break counter*2;
        }
    }
    }else if selected_option.trim()== "2" {
         loop_wit_lables() //why we dont write a semicolon here? because we want to return the value of the function, if we write a semicolon, it will return () which is the unit type, and we want to return the value of the function which is an i32 integer.
        }
        else if selected_option.trim()== "3" {
            counter_using_while_loop()
        }
        else if selected_option.trim()== "4" {
            counter_using_for_loop()
        }
             else {
            println!("Invalid option selected, defaulting to loop with labels");
            0
        };
    println!("The of loop counter result is: {result}");
        
    let a = [10, 20, 30, 40, 50];
for elem in a{
    println!("The element in the array is: {elem}");
}
    
}

fn loop_wit_lables()->i32 {
    let mut counter = 0;
    let result = 'loopName: loop{
        counter += 1;
        if counter%3==0{
            println!("The counter is divisible by 3: {counter}");
        } if counter==1000000{
            break 'loopName counter*3;
        }
    };
    result
}

fn counter_using_while_loop()->i32{
        let mut counter =0;

 'while_loop: while counter<=1000000{
            counter+=1;
        if counter%5==0{
                println!("The counter is divisible by 5: {counter}");
            }
  

}
       counter*5

}

fn counter_using_for_loop()->i32{
    for i in 0..=1000000{
        
        if i%7==0{
            println!("The counter is divisible by 7: {i}");
        }
    }
    1000000*7
}   

 