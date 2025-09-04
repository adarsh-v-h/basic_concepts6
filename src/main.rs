#[derive(Debug)] // this a attribute, which tells the compiler to implement its traits automatically

struct Rectangle{
    _width: u32,
    _height: u32,
}
fn main() {
    println!("Hello, friend!");
    let rect1 = Rectangle{
        _width: 30,
        _height: 50,
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
    dbg!(rect1); // this prints the file, line number of where dbg! marco call occurs in code, and returns ownership of the value,
    //println!("{}",rect1._height); this is invalid, because dbg! will take away the ownership
    // dbg! doesnt just prints, it retunrs the value, therefore we can use it to assign values too,
    let scale = 2;
    let rect2 = Rectangle{
        _width: dbg!(30* scale), // takes the expression and returns it, which is being assigned to width
        // it also prints the expression, "[src\main.rs:33:17] 30* scale = 60",
        // scale doesnt loses its value, as its an int, the macro will be using the copy function, to initialize width, not actually move scale's value
        _height: 50
    };
    //dbg!(rect2); // prints this too, and it loses its value
    // to avoid losing value of rect2(instance of struct), we can pass it as reference
    dbg!(&rect2); // passing reference
    println!("i can still use rect2 after {}", rect2._width); // rect2 is still valid 

}
