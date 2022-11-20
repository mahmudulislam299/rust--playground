
pub struct Cylinder 
{
	pub radius: f32,
	pub height: f32,
	pub unit: String,
}


pub trait Formula
{
	fn get_area(&self) -> f32;
	fn get_perimeter(&self) -> f32;
}

impl Formula for Cylinder
{
	fn get_area(&self) -> f32
	{
		return 2.0*3.1416* self.radius * (self.radius + self. height);
	}

	fn get_perimeter (&self) -> f32
	{
		return 2.0 * 3.1416 * self.radius;
	}
}


impl Cylinder 
{
	pub fn new(radius_n: f32, height_n: f32) -> Cylinder
	{
		Cylinder 
		{
			radius: radius_n,
			height: height_n,
			unit: "cm".to_string(),
		}
	}

	pub fn get_summary(&self)
	{
		println!("Summary:");
		println!("r: {} {}, h: {} {}", self.radius, self.unit, self.height, self.unit);
		println!("area is {} and perimeter is {}", self.get_area(), self.get_perimeter());
	}
}