// parameter

fn print_value(x: i32)
{
    println!("the value of x is: {}", x);
}

fn get_five() -> i32
{
    let x = 5;
    x
}

fn get_return_tuple(tup: (i32, f32, u8))
{
    for i in tup.iter()
    {
        println!("{}", i);
    }
}


fn main()
{
    println!("hello world");
    print_value(12);
    print_value(get_five());
    
    
    let tup: (i32, f32, u8) = (1222, 2.3, 180);
    get_return_tuple(tup);
}

