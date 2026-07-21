
#[derive(Debug)]
enum UsStates{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii
}
// As an example, let’s change one of our enum variants to hold data inside it. From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side. No other coins got state designs, so only quarters have this extra value. We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside it, which we’ve done in Listing 

enum Coins {
    Penny,
    Nickle,
    Dime,
    Quarter(UsStates),// cause its diffs in each state in us
}


// Concise Control Flow with if let and let...else
// we use this when we concern only with one State of variant
// we learned hhow to use mathces to check Enum Variants
fn main() {
    // ;let strat with example:
    let configMax = Some(3u8);// its Option<3u8>
    
    match configMax{//its check the value of ConfigMax
        Some(max)=>println!("the maximum number is: {max}") , // it will replace the value of max with 3
        _=>(),
    };// as shown we only concern only with one value, and we ignore the _ 
    // but we must to rite it because with out it we connot compile this code
    // that s we call it Boilerplate its required.

    // so what the solution ?
    // we gonna use if let like this:
    // if let Pattern = Expression.
    if let Some(max) = configMax{ // its similer to match if there is value in max it will access to expression
        //if there no value as NONE it will not goes into the block and it willnot do any thing
        println!("the maximum number is: {max}")
    };

 
// use if let{} else{}
let mut count = 0;
if let Coin::Quarter(state)=coin{
        println!("State quarter from {state:?}");
    
}else{
    count+=1;
}

// Using if let means less typing, less indentation, and less boilerplate code.
// but here we lose exhaustive checking, cause we cover only one state or one variant  not all variants

}
