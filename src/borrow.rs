// - Все данные сохраняемые в стеке (stack) должны быть известны и иметь фиксированный размер.
// - Данные с неизвестным размером во время компиляции, или размером который может изменится в ходе
//   выполнения программы должны сохраняться в куче (heap).
//
// Операционная система находит пустой участок кучи (heap), являющийся достаточно большим,
// помечает его как используемый и возвращает указатель, который является адресом данного участка.
//
// Данный процесс называется выделением в куче (allocation on heap).
// Размещение значений в стеке (stack) не считается выделением.
//
// Правила владения (borrow):
// 1. каждое значение имеет переменную, которая называется владельцем значения.
// 2. у значения может быть только один владелец в один момент времени.
// 3. когда владелец покидает область видимости, значение удаляется (drop).

pub fn run() {
    let x = 5; // can be stored on the stack, so it implements the "Copy" trait
    let y = x;

    println!("{x} {y}");

    // Когда мы назначили s1 переменной s2, то данные типа String были скопированы, что означает
    // что мы скопировали указатель, длину и ёмкость, которые находятся в стеке (stack).
    // Мы не копируем данные в куче (heap) на которые ссылается указатель.
    let s1 = String::from("hello"); // allocate memory on heap
    let s2 = s1;

    println!("s2: {s2}");
    // Вместо попытки копировать выделенную память, Rust считает что переменная s1
    // больше недействительна и таким образом в Rust ничего не нужно освобождать позже.
    // По-этому Rust не даст использовать недействительную ссылку s1
    //
    // println!("{s1}"); // ошибка: "borrow of moved value"

    // -- 1. Function can make a copy of a variable which is stored on the stack,
    // it will be still avaliable after function call
    fn make_copy(some_int: i32) {
        println!("some_int: {some_int}");
    }
    make_copy(y);
    println!("y: {y}");

    // -- 2. Function can take ownership, at the end of this function
    // memory allocated on the heap for that variable is freed.
    fn take_ownership(some_string: String) {
        println!("take_ownership prints: {some_string}");
    }
    take_ownership(s2);
    // println!("{s2}"); // ошибка: "borrow of moved value"

    // -- 3. Function can return ownership of a variable
    fn give_ownership() -> String {
        let some_string = String::from("world");
        some_string
    }
    let s3 = give_ownership();
    take_ownership(s3);
    // println!("{s3}"); // ошибка: "borrow of moved value"

    // -- 4. Function can take reference pointer of a mutatable variable and change it
    fn change(some_string: &mut String) {
        some_string.push_str(" or no?");
    }
    let mut s4 = String::from("yes");
    change(&mut s4);
    println!("s4: {s4}");
}
