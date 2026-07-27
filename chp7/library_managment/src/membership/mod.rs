    pub enum UserRole{
        Member,
        Guest,
    }
    pub struct User{
        pub userName:String,
        id : u64,
    }

    impl User{
        pub fn new(userName:&str ,id:u64)->Self{
            Self{
                userName: userName.to_string(),
                id,
            }
        }
    }