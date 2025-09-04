#[derive(Debug)] // this a attribute, which tells the compiler to implement its traits automatically
struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{ // rect1 will be as self and rect2 will be as other, both will be of type Rectangle, but in method the fields of rect1 will be called type self,

        self.width > other.width && self.height > other.height
    }// will return true if both conditions are true
}
fn main() {
    println!("Hello, friend!");
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    // if we want to print the whole "Rect1", we cant it with normal {} of println!
    // we are supposed to use :? inside the flower backets, this specifier tell println! we want to user output format called Debug
    /*The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code. 
    */
    println!("Instance of Struct Rectange: {:?}", rect1);
    // the structure will be like this, "{ _width: 30, _height: 50 }", it has the field names with curly braces
    // but we can make it loop better
    println!("Better looking: {:#?}", rect1);
    /* Rectangle {
    _width: 30,
    _height: 50,
} 
    it will print like this*/
    // The println! macro which takes the reference of the values to print them, 
    // we also have dbg! macro which takes the ownership of the expression
    //dbg!(rect1); // this prints the file, line number of where dbg! marco call occurs in code, and returns ownership of the value,
    //println!("{}",rect1._height); this is invalid, because dbg! will take away the ownership
    // dbg! doesnt just prints, it retunrs the value, therefore we can use it to assign values too,
    let scale = 2;
    let rect2 = Rectangle{
        width: dbg!(30* scale), // takes the expression and returns it, which is being assigned to width
        // it also prints the expression, "[src\main.rs:33:17] 30* scale = 60",
        // scale doesnt loses its value, as its an int, the macro will be using the copy function, to initialize width, not actually move scale's value
        height: 50
    };
    //dbg!(rect2); // prints this too, and it loses its value
    // to avoid losing value of rect2(instance of struct), we can pass it as reference
    dbg!(&rect2); // passing reference
    println!("i can still use rect2 after {}", rect2.width); // rect2 is still valid 
/*In rust we have something called "automatic referencing and dereferencing".
Here’s how it works: when you call a method with object.something(), Rust automatically adds in & , &mut , or * so object matches the signature of the method.
In other words, the following are the same:
    p1.distance(&p2);
    (&p1).distance(&p2);
This works because methods have clear receivers - type of self.*/
// lets create a method thats takes parameter, and lets make it check if 1 rectange fits into the another,
// will return true if does or false, we will create the 2 instances of Rectangle and then pass them
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // both are being passed as reference, even tho rect1 doesnt have &, the Rust will complie as if it has, because its already defined in function "&self",


}
