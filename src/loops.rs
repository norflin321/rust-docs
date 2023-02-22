pub fn run() {
    let mut count = 0;

    // Infinite Loop
    let result = 'counting_loop: loop {
        count += 1;
        println!("Number: {}", count);
        if count == 5 {
            break 'counting_loop count;
        }
    };

    // While
    while count <= 10 {
        println!("{}", count);
        count += 1;
    }

    // For
    for x in (0..5).rev() {
        println!("x: {}", x);
    }
}
