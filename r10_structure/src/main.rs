#![allow(dead_code)]

#[derive(Debug)]

struct Person
{
    name: String,
    age: u8,
}

// a unit struct
struct Unit;

// a tuple struct
struct Pair(i32, i32);

// a struct with two fields
struct Point
{
    x: f32,
    y: f32,
}

// struct can be reused as filed of another struct
struct Rectangle
{
    top_left:Point,
    bottom_right:Point,
}

fn main()
{
    let name = String::from("mahmudul");
    let age = 24;
    let mahmudul: Person = Person { name: name, age: age };

    // print debug struct
    println!("{:?}", mahmudul);
}

