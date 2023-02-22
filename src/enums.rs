// Enums are types which have a few definite values

pub fn run() {
    // value can be optional, this enum already predefined
    let absent_number: Option<i32> = None;

    // Message can be any of these types
    #[derive(Debug)]
    enum Message {
        Quit,                    // пустой элемент без ассоциированных данных
        Move { x: i32, y: i32 }, // имеет именованные поля, как и структура
        Write(String),           // элемент с единственной строкой типа String
        Color(i32, i32, i32),    // кортеж из трёх значений типа i32
    }

    impl Message {
        fn make(&self) -> String {
            String::from("a random msg")
        }
    }

    let msg = Message::Write(String::from("hello from me"));
    println!("msg: {:?}", msg);
    println!("made msg: {}", msg.make());

    // match
    match msg {
        Message::Quit => println!("message has 'Quit' type"),
        Message::Move { x: _, y: _ } => println!("message has 'Move' type"),
        Message::Write(_) => println!("message has 'Write' type"),
        other => println!("message has other type: {:?}", other),
    }

    // if let
    let msg2 = Message::Color(0, 0, 0);
    if let Message::Color(r, g, b) = msg2 {
        println!("message has 'Color' type: {:?}", (r, g, b));
    } else {
        println!("idc");
    }
}
