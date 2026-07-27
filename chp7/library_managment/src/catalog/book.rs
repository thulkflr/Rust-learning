      pub fn add_book() {
        //    use crate::audit::log_activity;
        //    Or
        use super::super::audit::log_activity;
           log_activity();
        }
        pub fn search_book() {
            println!("this is for search a book");
        }