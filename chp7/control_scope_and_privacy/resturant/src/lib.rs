pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


// we are in a crate root and its library crate
// lets create a mods of resturant 
// each mod can have multiple mods, structs or funcstions
mod front_of_house{
pub mod hosting{
   pub fn add_to_waitlist() {}

        fn seat_at_table() {}
}
mod serving{
    fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
}


}


fn deliver_order() {}

mod back_of_house{
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // "super" here is the parent mod "back_of_house"
        // dbg!(super);
    }

    fn cook_order() {}
}

fn eat_at_resturant(){
    crate::front_of_house::hosting::add_to_waitlist();// abs paht
    //   --> src/lib.rs:37:28
//    |
// 37 |     crate::front_of_house::hosting::add_to_waitlist();// abs paht
//    |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
//    |                            |
//    |                            private module
    // front_of_house::hosting::add_to_waitlist();// relative paht

    // we add pub keword and this error thrown:
    //    |
        // 37 |     crate::front_of_house::hosting::add_to_waitlist();// abs paht
        // |                                     ^^^^^^^^^^^^^^^ private function
        // |
        // note: the function `add_to_waitlist` is defined here
        // --> src/lib.rs:22:5
        // |
        // 22 |     fn add_to_waitlist() {}
        // |     ^^^^^^^^^^^^^^^^^^^^

        // For more information about this error, try `rustc --explain E0603`
// add a pub keyword to the funcrion add_to_waitlist()




}