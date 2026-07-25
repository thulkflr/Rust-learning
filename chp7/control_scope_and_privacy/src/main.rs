fn main() {
    println!("Hello, world!");
}
// Control Scope and Privacy with Modules

Mudule System:
1. Modules and Pathes to names the elements
2. "use" Keyword to brings a path into scope
3. "pub" Keyword for publiciate elements/
4. "as"  Keyword to renaim the elements when "use" elements To avoid name confusion

Modules Cheat Sheet:
this is the explain how the compiler "rustc" reads the project
1. starts with crate root
 - usually, src/lib.rs at Library Crate.
 - orm src/main.rs at Binary Crate.

2. Declaring Modules:
 When you write a command like `mod garden;` in the root file to declare a new module named `garden`, the compiler looks for the module's code in one of three places:
 - Directly in the same file (Inline): Inside curly braces `{}` written directly after `mod garden`.
 - In a separate file: In the path `src/garden.rs`.
 - In an older/followed directory: In the path `src/garden/mod.rs` (a common but relatively old approach).

3. Declaring Submodules:
 in any amother file  neither root file like we call it 'mod vegitables;'
 - Directly in the same file (Inline): Inside curly braces `{}` written directly after `mod vegitables`.
 - inside subfile  in this path "src/garden/vegetables.rs"
 - in subdirectory called mod.rs at "src/garden/vegetables/mod.rs"
 
4. Private vs. Public
 By default the code in any Module is Private to the parents Modules,
 to make it public we use pup mod instead of mod only.

5. use keyword:
 we can for example call the Asparagus like that:
 directlu "use Asparagus", instead of "crate::garden::vegetables::Asparagus;"


 we create a crate named backyard
 it will be like this:
 backyard
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── garden
        │   └── vegetables.rs
        ├── garden.rs
        └── main.rs //root crate



Grouping Related Code
 lest start with a resturant example:
 1. create new resturant librrary         
    --cargo new resturant --lib 
 2. write this hirarichy
    we separate the resturant to 2 parts 
    a. Front of House:
        mod front_of_house {
            mod hosting {
                fn add_to_waitlist() {}

                fn seat_at_table() {}
            }
            mod serving {
                fn take_order() {}

                fn serve_order() {}

                fn take_payment() {}
            }
        }

    b. Back of House

  and the tree result is:  
    crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
        └── serving
            ├── take_order
            ├── serve_order
            └── take_payment

    hosting and serving are siblings inside front of house parent        
    

