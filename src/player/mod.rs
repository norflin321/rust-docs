use crate::{traits::Movable, utils::*};

#[derive(Debug)]
pub enum PlayerAction {
	Move(Option<Point<f32>>),
	Talk(String)
}

#[derive(Debug)]
pub struct Balance {
	energy: u32,
	pub coins: u32,
	pub gems: u32
}

#[derive(Debug)]
pub enum Sex {
	Male,
	Female
}

#[derive(Debug)]
pub struct Player {
	pub lvl: u32,
	pub balance: Balance,
	pub sex: Sex,
	pub pos: Point<f32>,
}

impl Player {
	pub fn new() -> Self {
		let balance = Balance{ energy: 100, coins: 0, gems: 0 };
		Player { lvl: 0, balance, sex: Sex::Male, pos: Point::zero() }
	}

	pub fn get_energy(&self) -> u32 {
		self.balance.energy
	}

	pub fn do_action(&mut self, action: &PlayerAction) {
		if let PlayerAction::Move(p) = action {
			let by_pos = p.clone().unwrap_or(Point::zero());
			self.move_by(by_pos.x, by_pos.y); // calling a method implemented in trait Movable
			return;
		}
		if let PlayerAction::Talk(text) = action {
			self.do_talk(text.to_string());
			return;
		}
	}

	pub fn do_talk(&mut self, text: String) {
		println!("Player talk: {}", text);
	}
}