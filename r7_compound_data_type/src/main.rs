// Compound types can group multiple values into one type
// Rust has two primitive compound types: tuples and arrays.



fn main() 
{
    //**************************  The Tuple Type *********************************************//

    // A tuple is a general way of grouping together a number of values 
    // with a variety of types into one compound type. 
    // Tuples have a fixed length: 
    // once declared, they cannot grow or shrink in size.



    // We’ve added optional type annotations in this example

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // This is called destructuring, because it breaks the single tuple into three parts
    let (x, y, z) = tup;
    println!("the value of tup is {}", x);



    // we can access tupple elements directly by a period (.)

    let five_hundered = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("the value of tupple are {} {} {}", tup.0, tup.1, tup.2);




}
