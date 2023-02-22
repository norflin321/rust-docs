// Primitive str = Immutalbe fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own data

pub fn run() {
    // Primitive str is immutalbe
    let hello_str = "Hello";

    // String heap-allocated data structure, can be changed
    let mut hello = String::from("Hello ");

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Get length
    println!("{} has length of {}", hello, hello.len());

    // Get capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through strin gby whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());

    println!("{}", s);
}
