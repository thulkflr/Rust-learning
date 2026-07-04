//The Slice Type
// A slice is a reference to a contiguous sequence of elements in a collection rather than the whole collection.
// Slices let you reference a section of an array or a string without having to copy it

fn main() {
    let mut s= String::from("hello world");
    let word=first_word(&s);// we are passing a reference to s to the function first_word, so that we can use s after the function call
    dbg!(&s);// this will work because s is still valid after the function call, because we passed a reference to it instead of moving it into the function
    dbg!(&word);// this will work because word is a new variable that has taken ownership of the value that was returned from the function first_word
    s.clear();// this will work because s is still valid after the function call, because we passed a reference to it instead of moving it into the function
    //
    // However, this will cause a problem because we are trying to use s after it has been cleared, 
    // which means that the value of s is no longer valid. This is because the function

    //A string slice is a reference to a contiguous sequence of the elements of a String, and it looks like this:

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    dbg!(&hello);
    dbg!(&world);
    // The first slice, &s[0..5], contains the first five bytes of the string, which are the bytes for the word "hello".
    // The second slice, &s[6..11], contains the next five bytes of the string, which are the bytes for the word "world".
    // The two slices are references to the same String, but they are different slices of that String. The first slice is a reference
    // to the first five bytes of the String, and the second slice is a reference to the next five bytes of the String.

    let s = String::from("hello world");
    let slicesString = first_word_with_slice(&s);
    dbg!(&slicesString);
    
        if false{
            let mut testS= String::from("hello world");
            let slicesStrin1g=first_word_with_slice(&testS);
            testS.clear();// this will work because testS is still valid after the function call, because we passed a reference to it instead of moving it into the function
            // dbg!(&slicesString);// this will throw an error because slicesString is a reference to a part of testS, and testS has been cleared, so slicesString is no longer valid after the function call
            // println!("The first word is: {slicesString}");// this will throw an error because slicesString is a reference to a part of testS, and testS has been cleared, so slicesString is no longer valid after the function call 
            // dbg!(&slicesStrin1g);}
        }


        //String Literals as Slices
        // String literals are slices, so they are references to a contiguous sequence of characters in a string.
        let s = "hello world";
        let hello = &s[0..5];
        let world = &s[6..11];
        dbg!(&hello);
        dbg!(&world);


        //String Slices as Parameters
        let my_string = String::from("hello world");
        // first_word works on slices of `String`s, whether partial or whole
        let wordi = first_word_with_slice_str(&my_string[0..6]);
        let wordi = first_word_with_slice_str(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent to
        // whole slices of `String`s
        let word = first_word_with_slice_str(&my_string);


        let wordi = first_word_with_slice(&my_string[0..6]);//it will return an error because
        // the function first_word_with_slice expects a reference to a String, but we are passing a reference to a string slice.

        
        dbg!(&wordi);



}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    //because we need to go through the string byte by byte, we can convert the string to an array of bytes using the as_bytes() method. This
    // method returns a slice of the string as an array of bytes, which we can then iterate over to find the first space character.
    for (i, &item) in bytes.iter().enumerate() {
        // For now, know that iter is a method that returns each element in a collection
        // and that enumerate wraps the result of iter and returns each element as part of a tuple instead.
        // The first element of the tuple returned from enumerate is the index of the element in the collection, and
        // the second element is a reference to the value of the element itself.
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn first_word_with_slice(s: &String)->&str{
    //&str is a string slice, which is a reference to a contiguous sequence of characters in a string. 
    //its similar to &String, but it is a reference to a string slice rather than a reference to a String.
    let byte= s.as_bytes();
    for (i,&item) in byte.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_with_slice_str(s: &str) ->&str{
      let byte= s.as_bytes();
    for (i,&item) in byte.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
        }
    }
    &s[..]
}