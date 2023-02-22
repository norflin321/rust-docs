// Tuples (кортежи) group together values of different types
// Max 12 elements
//
// Имеют фиксированную длину и могут иметь значения различного типа

pub fn run() {
    let person = ("Brad", "Mass", 37);

    let (name, from, age) = person;

    println!("{name} is from {from} and is {age}");
}
