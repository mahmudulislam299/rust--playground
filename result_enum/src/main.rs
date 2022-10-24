// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn check_if_five(number: i32) -> Result <i32, String>
{
    match number
    {
        5 => Ok(number),
        _ => Err("sorry the number is not five".to_string())
    }
}

fn main()
{
    let mut result_vec = Vec::new();

    for number in 2..9
    {
        result_vec.push(check_if_five(number));
    }


    for item in result_vec
    {
        if item.is_ok()
        {
            println!("{:?}", item.unwrap());
        }
        else
        {
            println!("{:?}", item);
        }
    }

    // for vector in result_vec
    // {
    //     println!("{:?}", vector);
    // }
}