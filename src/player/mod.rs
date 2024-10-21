#[derive(Debug, Clone)]
pub struct Point { pub x: i32, pub y: i32 }
impl Point {
	pub fn zero() -> Self {
		Self{ x: 0, y: 0 }
	}
}

#[derive(Debug)]
pub enum PlayerAction {
	Move(Option<Point>),
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
	pub pos: Point,
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
			self.do_move(p.clone().unwrap_or(Point::zero()));
			return;
		}
		if let PlayerAction::Talk(text) = action {
			self.do_talk(text.to_string());
			return;
		}
	}

	pub fn do_move(&mut self, p: Point) {
		println!("Player move: {:?}", p);
		self.pos.x += p.x;
		self.pos.y += p.y;
	}

	pub fn do_talk(&mut self, text: String) {
		println!("Player talk: {}", text);
	}
}