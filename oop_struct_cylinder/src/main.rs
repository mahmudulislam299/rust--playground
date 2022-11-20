pub mod cylinder_folder;
use cylinder_folder::cylinder_file::{Cylinder,Formula};


fn main() {
    println!("Hello, world!");

    let cylinder1 = Cylinder{
        radius: 1.00, 
        height: 2.00,
        unit: "cm".to_string(),
    };

    println!("radius of cylinder: {} {}", cylinder1.radius, cylinder1.unit);
    println!("height of cylinder: {} {}", cylinder1.height, cylinder1.unit);

    println!("area is {} and perimeter is {}", cylinder1.get_area(), cylinder1.get_perimeter());




    println!("another call");

    let cylinder2 = Cylinder::new(3.0,5.0);
    cylinder2.get_summary();
}
