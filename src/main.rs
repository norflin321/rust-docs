use std::{cmp::Ordering, io, ops::RangeInclusive};
use std::collections::HashMap;
use rand::Rng;

// Rust project structure (https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html):
// package = consists of one or more crates and has one Cargo.toml.
// crate = binary (src/main.rs, produces executable) | library (has lib.rs, no executable).

// when we declare a new module, compile will look for it's code in these places: "./utils.rs", "./utils/mod.rs".
// once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate.
pub mod utils;
pub mod player;

// "use" keyword creates shortcuts to reduce repetition of long paths
pub use crate::utils::{calc_length, get_slice};
pub use crate::player::*;

const RANGE: RangeInclusive<i32> = 0..=1;

fn main() {
	let arr = [0, 1]; // arrays allocated on the stack
	for n in arr {
		println!("{}", n);
	};

	let rand_n = rand::thread_rng().gen_range(RANGE);

	// "!" means calling a macro
	println!("try to guess a number in range {:?}", RANGE);

	// variables are immutable by default, "new()" is a function that is implemented on a type String (growable UTF-8 encoded text)
	let mut i_buf = String::new();
	loop {
		// "&" means taking a reference, they are also immutable by default.
		// Result is an enum with Ok and Err variants, and it has some methods defined on it.
		io::stdin().read_line(&mut i_buf).unwrap();
		let guess = i_buf.trim().parse::<i32>().expect("Error: you should provide an i32");

		// compare random number with user input, using "match" on enum, break loop when equal
		match guess.cmp(&rand_n) {
			Ordering::Equal => {
				println!("You win!");
				break;
			},
			_ => ()
		};

		i_buf.clear();
	}

	// Ownership Rules:
	// 1. Each value in Rust has an owner.
	// 2. There can only be one owner at a time.
	// 3. When the owner goes out of scope, the value will be dropped.
	let s1 = String::from("hello");

	// When assigning/passing/returning:
	// 1. values that are stored on the stack do copy.
	// 2. values that are stored on the heap do move (or we can call clone() method).
	let mut s2 = s1;

	// println!("{s1}"); // error: s1 is no longer valid, because pointer moved from s1 into s2
	println!("{s2}");

	// references (&) allows to refer to a value without moving ownership
	println!("{}", calc_length(&mut s2)); // we call the action of creating a reference - "borrowing"
	println!("{}", get_slice(&s2));
	// 1. we can not have a mutable reference while we have an immutable one
	// 2. we can not have two mutable references

	let mut player = Player::new();
	println!("{player:?}");
	println!("player energy: {}", player.get_energy());

	// collections (Vec, String, Map, HashMap, ...) are stored on the heap which means the amount of data
	// does not need to be known at compile time and can grow or shrink as the program runs. collection with
	// all of its content is freed when it goes out of scope.
	let mut actions = vec![
		PlayerAction::Talk(String::from("hello")),
		PlayerAction::Move(Some(Point{ x: 5, y: 5 })),
	];

	// maybe move again?
	let mut distance: Option<Point> = None;
	if rand::thread_rng().gen_range(0..=1) == 0 {
		distance = Some(Point{ x: 10, y: -5 })
	}
	actions.push(PlayerAction::Move(distance));

	// do actions
	for action in &actions {
		player.do_action(action);
	}
	println!("actions: {:?}", actions);
	dbg!(&player);

	let mut scores: HashMap<String, i32> = HashMap::new();
	scores.insert(String::from("a"), 5);
	scores.insert(String::from("b"), 10);
	scores.entry(String::from("b")).or_insert(20);
	for (k, v) in scores {
		println!("{k}: {v}");
	}
}