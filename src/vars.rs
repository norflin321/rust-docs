// Variables hold primitive data or references
// Variables are immutable by default
// Rust is a block-scoped language
pub fn run() {
    let name = "Brad";
    let mut age = 37;
    age = 38;

    println!("My name is {} and my age is {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
