struct People {
    name: &'static str,
    age: u8,
}

fn main() {
    let people = People {
        name: "Lala",
        age: 29,
    };

    println!("People name: {}, age: {}", people.name, people.age);
}
