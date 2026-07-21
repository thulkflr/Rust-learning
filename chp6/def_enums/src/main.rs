//Enums and Patern Matching
// Enums allow you to define a type by enumerating(تعداد) its possible variants.يعني بامكانا نعدد احتمالات المتغير الواحد
// as we learned in Struct that is the way to group related fields together,
// enums give us a way of saying a value is one of a possible set of values
// for example we may want to say that rectangle is one of a set of possible shapes 
// that also include Circle and Triangle. to do this , RUST alllows us to 
// encode these posslibilities as an enum.

// let s do an example to Proof that enums are better than STructs
// Evreyone Knows that IP addresses are globaly known in two types
// IPV4 and IPV6, these are the possibilities of IP addresses so we enumerate all possible variants
// evry IP address can be V4 or V6 not both at the same time, 
// That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants.

// we can express this concept in code by Define this ENum:
// 

enum IPAdressesVersions{
    V4,
    V6,
}

enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32 )
}

struct QuitMsg;
struct MoveMsg{
    x: i32,
    y: i32,
}

struct WriteMsg(String);

struct ChangeColorMsg(i32,i32,i32);


struct IpAddress{
    ip_type:IPAdressesVersions
 ,  address: String
}

// we can implements the Enum Like struct:
impl Message{
    fn callMsg(&self)
    {
        self
    }
}
//in struct we call the content of struct : Fields
//in Enums its called : Variants.

// The Option Enum
// https://claude.ai/share/e420c5b3-9722-42e6-be83-f2f9531bf6f3
// For example, if you request the first item in a non-empty list, you would get a value. If you request the first item in an empty list, 
// you would get nothing ,this functionality can prevent empty data bugs.
// >>>> Rust donsnt have the null feature , in other languages the variables can be null or not null like PHP.
// However, the concept that null is trying to express is still a useful one: A null is a value that is currently invalid or absent for some reason.
// keep this in ur mind , Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.
// the enum is Option<T> is defined by the standard library as follows:

//Option<T> included in prelude so we dont need to import it
enum Option<T>{//T is generic type parameter
    None,// There is No Nulls in RUst , None Means thats no data here and it also included in prelude
    Some(T), // there is data stores in Some(T) and it also included in prelude
}

fn main() {
// we can use Enums Like this:
let IPV4= IPAdressesVersions::V4;
let IPV6= IPAdressesVersions::V6;
usingEnumArg(IPV4);


//using struct instance
let home = IpAddress{
    ip_type:IPAdressesVersions::V4,
    address: String::from("192.168.1.1")
}

println!("{IPV4}");


// <T> means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets
// used in place of T makes the overall Option<T> type a different type. Here are some examples of using Option values to hold number types and char types:
let someNumber = Option::Some(5);
let someNumberSome = Some(5);//are the same because Some included in the prelude

let someChar = Some('c');

let absentNumber: Option<i32>=None; // same of null and  <i32> is <T>

// why options better than Null?
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;  // the compiler dosent compile this and it will show an error
    //this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>
    // When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value.
    // Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.





}


fn usingEnumArg (typeIp: IPAdressesVersions )
{
    typeIp::V4
}
