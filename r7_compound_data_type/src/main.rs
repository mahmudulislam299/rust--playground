// Compound types can group multiple values into one type
// Rust has two primitive compound types: tuples and arrays.

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

fn main() 
{
    //**************************  The Tuple Type *********************************************//

    // A tuple is a general way of grouping together a number of values 
    // with a variety of types into one compound type. 
    // Tuples have a fixed length: 
    // once declared, they cannot grow or shrink in size.



    // We’ve added optional type annotations in this example

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup2 = (101, 56.6, 'c');

    // This is called destructuring, because it breaks the single tuple into three parts
    let (x, _y, _z) = tup;
    println!("the value of tup is {}", x);



    // we can access tupple elements directly by a period (.)

    let _five_hundered = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
    println!("the value of tupple are {} {} {}", tup.0, tup.1, tup.2);


    //**************************  The Array Type *********************************************//

    // Arrays are useful when you want your data allocated on the stack rather than the heap

    let a = [1,2,3,4,5];

    // You write an array’s type using square brackets with the 
    // type of each element, a semicolon, and then the number of 
    // elements in the array, like so:
    let _b:[i32;5] = [1,2,3,4,5];

    let _c = [3;5];
    // he array named a will contain 5 elements that will all be set to the value 3 initially. 
    // This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.


    // accessing array elements
    let _first = a[0];
    let _second = a[1];



}
