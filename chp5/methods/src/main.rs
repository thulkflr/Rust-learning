// Methods
// methods are similar to functions, but they are associated with a struct or an enum. 
// they are defined within an impl block, which is short for implementation block.
// there first parameter of a method is always self, which
// represents the instance of the struct or enum that the method is being called on.
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle{
        height: 10,
        width: 5,
    };
     let rect2 = Rectangle{
        height: 8,
        width: 3,
    };
     let rect3 = Rectangle{
        height: 20,
        width: 15,
    };



    println!("The area of the rectangle is {}", rect1.area());
    println!("the width of rectangle is {}",&rect1.width());
    println!("the height of rectangle is {}",&rect1.height());

    println!("rect 1 can hold rect 2 {}",rect1.can_hold(&rect2));
    println!("rect 1 can hold rect 3 {}",rect1.can_hold(&rect3));
}



impl Rectangle {
    //every thing that is defined within an impl block is called an associated function to Rectangle, and we can call it using the dot notation on an instance of Rectangle.
    //the first parameter of a method is always self, which represents the instance of the struct or enum that the method is being called on.
    //we can use &self to borrow the instance of the struct or enum, which means that we can call the method on an instance of the struct or enum without taking ownership of it.
    //we can also use &mut self to borrow the instance of the struct or enum mutably, which means that we can call the method on an instance of the struct or enum and modify it.
    //we can also use self to take ownership of the instance of the struct or enum, which means that we can call the method on an instance of the struct or enum and move it into the method.       

    fn area(&self) -> u32 {
        //its same of self : &Self
        dbg!(&self);
        self.height * self.width
    }

    //  we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle that is also named width
    fn width(&self)-> bool{
        self.width > 0
    }

    fn can_hold(&self, ontherRect: &Rectangle)->bool{
        self.area()>ontherRect.area()
        
    }
    
}

    //Multiple impl Blocks
    impl Rectangle {
        fn height (&self) -> bool{
            self.height>0
        }
    }

// instead off 
    // fn area(rect:&Rectangle)->u32{
    //     rect.hieght*rect.width
    // }

