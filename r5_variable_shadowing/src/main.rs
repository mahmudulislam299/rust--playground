// We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:

fn main() 
{
    println!("Hello, world!");

    let x = 5;
    println!("the value of x: {}" , x);

    let x = x+1;
    println!("the value of x: {}" , x);

    {
        let x = x*2;
        println!("the value of x in inner scope is : {}" , x);
    }

    println!("the value of x: {}" , x);

}
