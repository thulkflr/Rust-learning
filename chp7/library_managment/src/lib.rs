// this is Library crate
pub use crate::catalog::*;
pub fn init() {

    println!("Hello from Library!");
}


pub mod catalog;

mod audit {
   pub  fn log_activity(){
            println!("this is for log activity");  
    }
}

pub mod membership; //it will search on membership.rs in src folder in new rust version
// in older versions it will search on mod.rs file in membership 