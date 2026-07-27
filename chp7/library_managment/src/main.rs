use library_managment::*;
// this is binary crate

fn main() {
init();
book::search_book();
catalog::book::add_book(); //its private



let name= membership::User::new("ahmad",1);
dbg!(&name.userName);
// user.id=5;
}
