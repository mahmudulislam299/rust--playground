// two data type subsets: scalar and compound.
// Rust is a statically typed language, which means that it must know 
// the types of all variables at compile time.


fn main() 
{
    println!("Hello, world!");

    /*-------------------------------------------------------------------- */
    // Floatin-point types
    
    // The f32 type is a single-precision float, and f64 has double precision

    let _x = 2.0; //f64
    let _y: f32 = 3.0; //f32

    /*-------------------------------------------------------------------- */

    // Numeric Operations

    let sum = 5+ 10;

    let difference = 95.6 - 5.5;

    let product = 4*2;

    let quotient = 56.7 / 32.2;

    let floored = 2/3;

    let remainder = 43 % 5;

    println!("sum is {} \ndiff is {} \nproduct is {} \nquotient is {} \nfloored is {}\nremainder is {}",
            sum, difference, product, quotient,floored, remainder);


    /*-------------------------------------------------------------------- */

    // boolean type

    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("t is {} and f is {}", t, f);

    /*-------------------------------------------------------------------- */

}
