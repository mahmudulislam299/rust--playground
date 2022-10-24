use core::num;
use std::num::ParseIntError;

fn return_number(input: &str) -> Result<i32, ParseIntError>
{
    return input.parse::<i32>();
}



fn main() 
{
    println!("Hello, world!");
    let my_number = return_number("250s");
    // if my_number.is_ok()
    // {
    //     println!("{:?}",my_number.unwrap());
    // }

    let mut got_number = 0;

    match my_number
    {
        Ok(number) => 
        {
            println!("{:?}",number);
            got_number = number.clone();
        },
        Err(ref e) => println!("err happened{:?}",e),
    }

    println!("{:?}",my_number);

    
    // if my_number.is_err()
    // {
    //     print!("this error");
    // }
}
