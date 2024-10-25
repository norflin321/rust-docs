use crate::player::*;

// we can use traits to define shared behavior in an abstract way (aka interfaces)
pub trait Movable {
	fn move_by(&mut self, x: f32, y: f32);

	// default implementation can be overwritten
	fn move_to(&mut self, x: f32, y: f32) {
		println!("i moved to {x}.{y}");
	}
}

// now that we implemented the trait on Player type, we can call the trait methods
// on instances of the Player type in the same way we call regular methods.
impl Movable for Player {
	fn move_by(&mut self, x: f32, y: f32) {
		self.pos.x += x;
		self.pos.y += y;
	}
	fn move_to(&mut self, x: f32, y: f32) {
		self.pos.x = x;
		self.pos.y = y;
	}
}