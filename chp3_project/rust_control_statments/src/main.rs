fn main() {
    let condition: bool = true;
    let numberForTest= if condition { 9 } else { 10 };
    println!("The value of numberForTest is: {numberForTest}");

    let mut selected_option : i32 =0;
    // conditiotions with loops
    let mut counter = 0;
    io::stdin().read_line(&mut selected_option).expect("Failed to read line");



    let result =if selected_option==1 { loop {
        counter += 1 ;
        if counter%2==0{
            println!("The counter is even: {counter}");
        } if counter==1000000{
            break counter*2;
        }
    };
    println!("The of loop counter result is: {result}");
    }else if selected_option==2 { loop_wit_lables(counter);
    println!("The of loop counter result is: {result}");

}

}

fn loop_wit_lables(counter: i32)-> i32 {
    let result = 'loopName: loop{
        counter += 1;
        if counter%3==0{
            println!("The counter is divisible by 3: {counter}");
        } if counter==1000000{
            break 'loopName counter*3;
        }
    };
}