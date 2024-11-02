use std::thread;
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
pub mod traits;

// "use" keyword creates shortcuts to reduce repetition of long paths
pub use crate::utils::*;
pub use crate::player::*;
pub use crate::traits::*;

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
	// 1. values that are stored on the stack do copy (if they implement "Copy" trait).
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
		PlayerAction::Move(Some(Point{ x: 5.0, y: 5.0 })),
	];

	// maybe move again?
	let mut distance: Option<Point<f32>> = None;
	if rand::thread_rng().gen_range(0..=1) == 0 {
		distance = Some(Point{ x: 10.0, y: -5.0 })
	}
	actions.push(PlayerAction::Move(distance));

	// do actions
	for action in &actions {
		player.do_action(action);
	}
	println!("actions: {:?}", actions);

	move_to_start(&mut player);
	dbg!(&player);

	let mut scores: HashMap<String, i32> = HashMap::new();
	scores.insert(String::from("a"), 5);
	scores.insert(String::from("b"), 10);
	scores.entry(String::from("b")).or_insert(20);
	for (k, v) in scores {
		println!("{k}: {v}");
	}

	// lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure
	// references will be valid in more situations than it could without our help. the main aim is to prevent dangling references.
	let str1 = String::from("Anton");
	let result;
	{
		let str2 = String::from("Kurtin");
		result = longest(str1.as_str(), str2.as_str());
		println!("{}", result)
	}

	// error! because we annotated the lifetimes of the function parameters and return values with the same lifetime ('a)
	// println!("{}", result)

	/// this lifetime annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field
	struct _ImportantExcerpt<'a> {
	  part: &'a str,
	}

	// three lifetime elision rules (the compiler uses them when there are no explicit lifetime annotations):
	//
	// 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
	// In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
	// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
	//
	// 2. The second rule is that, if there is exactly one input lifetime parameter,
	// that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32
	//
	// 3. The third rule is that, if there are multiple input lifetime parameters, but one of them is "&self" or "&mut self" because this is a method,
	// the lifetime of "self" is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

	let mut num = 1;

	// closures can capture values from their environment in three ways (which directly map to the three ways a function can take a parameter):
	// 1. borrowing immutably
	// 2. borrowing mutably
	// 3. and taking ownership
	// The closure will decide which of these to use based on what the body of the function does with the captured values.
	let mut add_num = |x: &mut i32| {
		num += 1; // causes the closure to captures a mutable reference of "num" and automaticly implement "FnMut"
		*x += num;
	};
	let mut a = 0;
	// println!("num: {}", num); // error: cannot borrow "num" as immutable because it is also borrowed as mutable
	add_num(&mut a);

	println!("num: {}", num);
	println!("a: {}", a);

	// closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
	//
	// 1. "FnOnce" applies to closures that can be called once. All closures implement at least this trait, because all closures can be called.
	// A closure that moves captured values out of its body will only implement "FnOnce" and none of the other "Fn" traits, because it can only be called once.
	// For example a closure can implement "FnOnce" when it capture a value and then transferring ownership of that value to another function that it calls,
	// trying to call the closure a second time wouldn’t work because value would no longer be in the environment that is was captured from.
	//
	// 2. "FnMut" applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
	//
	// 3. "Fn" applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

	let list = vec![1, 2, 3];
	// we need to specify that we want to move ownership of "list" to the closure (using "move" keyword), because if the main thread maintained ownership of "list"
	// but ended before the new thread did and dropped "list", the immutable reference in the thread would be invalid.
	thread::spawn(move || println!("from thread: {list:?}")).join().unwrap();
}