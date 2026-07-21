//Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
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

fn value_in_cents(coin:Coins)->u8{
    match coin{
        Coins::Penny    =>  1,//this is arm
        // and arm has two parts:
    // this is pattern  => this is code
    // Coin::Penny <-- Pattern
    // => 1 <--- code
        Coins::Nickle=>5,//this is arm
        Coins::Dime=>10,//this is arm
        Coins::Quarter(state)=>{//Patterns That Bind to Values
            println!("State Quarter from: {:?}",state);
            25
        },//this is arm
    }
    // the match arms. An arm has two parts: a pattern and some code. 
    // The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run.
    // The code in this case is just the value 1. Each arm is separated from the next with a comma.
} //https://chatgpt.com/share/6a56209d-e968-83eb-ac00-3738a0e03f8f


fn main() {

  println!("this is the resutl: {}",value_in_cents(Coins::Penny));
  println!("this is the resutl: {}",value_in_cents(Coins::Quarter(UsStates::Georgia)));
//   If we were to call value_in_cents(Coin::Quarter(UsState::Georgia)),
//   coin would be Coin::Quarter(UsState::Georgia).
//   When we compare that value with each of the match arms, none of them match until we reach 
//   Coin::Quarter(state). At that point, the binding for state will be the value UsState::Alaska.
//   We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.

// The Option<T> match Pattern
fn plus_one(option:Option<i32>)->Option<i32>{
    // it recives first Some(5)
     match option {
        None=>None,// Not match to Some(5)
        Some(i)=>Some(i+1), // Match to Some(5) so it will replac i with 5
        //Does Some(5) match Some(i)? It does! We have the same variant. The i binds to the value contained in Some, so i takes the value 5. The code in the match arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.
// https://chatgpt.com/share/6a56373d-df9c-83ed-bea4-6d276cdc741f
    }


}
     let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six : {}",six)




}
// Catch-All Patterns and the _ Placeholder
// https://chatgpt.com/share/6a5f899c-f24c-83eb-9d0e-eeecc0db8039