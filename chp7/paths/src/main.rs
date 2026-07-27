fn main() {
    println!("Hello, world!");
}
let s start with Paths for Referring to an Item in the Module Tree
to select a location of an element inside the Modules tree we use Path 
exactly how we use Pathes in Computers system Folders

there is two types of Paths and it separates by (::)
1. Absolute Path:
    it start from the Root Crate
    the path start with crate like this: 
    crate::front_of_house::hosting::add_to_wishlist();  

2. Relative Path:
    starts from current Module
    it use self keyword or super as identifire of current Module
    assume we are in or at front_of_house Module: 
    the relative path will be like this:
    front_of_house::hosting::add_to_wishlist();


        crate::front_of_house::hosting::add_to_waitlist();// abs paht
           --> src/lib.rs:37:28
            |
         37 |     crate::front_of_house::hosting::add_to_waitlist();// abs paht
            |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
            |                            |
            |                            private module

        this error will shown when we build the code! whyyy? it say is not publicly re-exported and hosting module is private.
        the elemnts thats included in childs, the parents cant access on it so we have to publicate the children Modules

        So, what happens if we add pub to hosting module?
        Oops! another Error!:
        |
     37 |     crate::front_of_house::hosting::add_to_waitlist();// abs paht
        |                                     ^^^^^^^^^^^^^^^ private function
        |
        note: the function `add_to_waitlist` is defined here
        --> src/lib.rs:22:5
        |
     22 |     fn add_to_waitlist() {}
        |     ^^^^^^^^^^^^^^^^^^^^

        For more information about this error, try `rustc --explain E0603`

        if we publicate the module thats make the access of parents only to child Module
        but still there is no access for content of the child Module. so the solution 
        add a pub keyword to the funcrion add_to_waitlist() and success build finally.

        Best Practis for packages that contains Binary and Library Crates:
         - Whole Project Modules and codes writes inside Library crate "src/lib.rs"
         - the "src/main.rs" is only for run the startup application code and it is simple
           Like RunApp() function in Yii2 its like a Public API for our project.


LETS START WITH RELATIVE PATHS USING super:
    fn deliver_order() {}

    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::deliver_order(); //we can start the path with super thats means we start the path from parent 
            // super --> (..) which means the parent of back_of_house let say its the crate or another Parent Module,
            // and this parent thing has our function deliver_order its seems to this (..)::deliver_order()
        }

        fn cook_order() {}
    }

    super here same to ".."

------------------------------------------------------------------------
pub keyword WITH Structs AND Enums:

1. if we add pub kw with struct thats make the struct public but the fields of struct still private
So we have to publicate the fields to make it usable.

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // public field
        seasonal_fruit: String, // private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    meal.seasonal_fruit = String::from("blueberries");// it will throw an error because we access on private field

}

2. if we add pub to enum all variants in enum will be public: 

mod back_of_house {
    pub enum Appetizer {
        Soup,  // Automatically public variant
        Salad, // Automatically public variant
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


-----------------------------------------------------------------------
Start with 7.4 section: Bringing Paths into Scope with the use Keyword
Instead of write the path of element when we need it evreywhere we ganna use the use Keyword once at time only
mod front_of_house
{
    pop mod hosting{
        pub fn add_to_wishlist();
    }
}
use crate::front_of_house::hosting;

pub fn eatAtResturant(){
    hosting::add_to_wishlist();

}

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();// it will show an error because we are in child module and we are out of range of the parant module
|       ^^^^^^^ use of unresolved module or unlinked crate `hosting`
warning: unused import: `crate::front_of_house::hosting` //its unused because there is no useage of this path in parent module
 --> src/lib.rs:7:5
    |
124 | use crate::front_of_house::hosting;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(unused_imports)]` on by default
    }// to fix that we can add super:: to access on parent and use the path
}

-----------------------------------------------------
Idiomatic use Paths
why we cant call  the only add_to_waitlist instead of using hosting::add_to_waitlist();?
like this:
 pub fn eat_at_restaurant() {
    add_to_waitlist();
} we need to call the Idiomatic use
use crate::front_of_house::hosting::add_to_waitlist;
thats will work
 ***  assume we have two modules or elenments in module or somting with the same name and we need to 
 use them in the same module, Like this:
 use crate::sami::somthing::Result;
 use mohammad::hello::Result;

 Rust will prevent the code for work, the solution of this problem
 is using "as" key word, like this:
 use crate::sami::somthing::Result;
 use mohammad::hello::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip-- 
}


-------------------------------------------------------
Re-exporting

