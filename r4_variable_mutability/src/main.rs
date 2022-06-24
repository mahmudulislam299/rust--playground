// by default variables are immutable
// When a variable is immutable, once a value is 
// bound to a name, you can’t change that value

fn main() 
{
    println!("Hello, world!");

    let x = 5;
    println!("the value of x is : {}", x);

    // this is not possible. bcoz by default variable is immutable
    // x = 6; 


    /* mutable variable */

    let mut y = 10;
    println!("the value of y is : {}", y);

    y= 20; // this is possible
    println!("the changed value of y is : {}", y); 
}
    
