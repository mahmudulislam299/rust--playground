#[derive(Debug)]
struct Rectangle 
{
    width: u32,
    height: u32,
}

impl Rectangle
{
    fn area(&self) -> u32
    {
        return self.width * self.height;
    }
}

fn main()
{
    let rect1 = Rectangle{ width: 50, height: 50};
    
    println!("THe area of rectangle is {} square pixels", rect1.area());
}