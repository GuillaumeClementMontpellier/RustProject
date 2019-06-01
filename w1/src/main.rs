#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle)-> bool{
		self.width > other.width && self.height > other.height
	}
}

fn main() {
	let rect1 = Rectangle { width: 30, height: 50 };
	let rect2 = Rectangle { width: 10, height: 60 };

	println!(
		"The area of the rectangle 1 is {} square pixels.",
		rect1.area()
		);

	println!(
		"The area of the rectangle 2 is {} square pixels.",
		rect2.area()
		);

	println!(
		"Can rectangle 1 hold rectangle 2 : {}",
		rect1.can_hold(&rect2)
		);

}
