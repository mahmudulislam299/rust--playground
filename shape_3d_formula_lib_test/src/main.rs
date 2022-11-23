use formula_3d_shape_lib::sphere::Sphere;
use formula_3d_shape_lib::solid::Solid;
use formula_3d_shape_lib::cylinder::Cylinder;


fn main() 
{
    println!("Hello, world!");

	let sphere1 = Sphere::new(4.0);
	sphere1.summary();


   
	let solid1 = Solid::new(4.0, 10.0,3.0);
	solid1.summary();



	let cylinder1 = Cylinder::new(4.0, 10.0);
	cylinder1.summary();
}
