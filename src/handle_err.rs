use std::fs::File;

pub fn run() {
    // Ошибки группируются на две основные категории:

    // -- 1. неисправимые (unrecoverable), используем: "panic!"
    // panic!("crash and burn");
    // По умолчанию, когда происходит паника, программа начинает процесс раскрутки стека.
    // "RUST_BACKTRACE=1" - error tracing (читать сверху вниз)

    // -- 2. исправимые (recoverable), используюем тип: "Result<T, E>"
    let f = File::open("readme.md");
    match f {
        Ok(file) => println!("{:?}", file),
        Err(e) => println!("error: {:?}", e),
    }

    // unwrap/expect
    // Если "Ok" то верется значение внутри, если "Err" то "panic!".
    let f = File::open("readme.md").unwrap();
    let f = File::open("readme.md").expect("Failed to open readme.md");
    println!("{:?}", f);

    // После типа "Result" можно поместить оператор "?", тогда если значение является
    // типом "Err" то функция остановит выполнения и вернет значение "Err"
    fn read_file() -> Result<File, std::io::Error> {
        let f = File::open("readme.md")?;
        Ok(f)
    }
}
