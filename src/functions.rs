pub fn run() {
    greeting("Hello", "Anton");

    let get_sum = add(1, 2);
    println!("Sum: {}", get_sum);

    // Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Sum: {}", add_nums(2, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
