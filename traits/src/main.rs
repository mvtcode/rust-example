trait ShowInfo {
    fn new(name: &'static str, age: u8) -> Self;
    fn show_name(&self);
    fn show_age(&self);
}

struct People {
    name: &'static str,
    age: u8,
}

struct Animal {
    name: &'static str,
    age: u8
}

impl People {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_age(&self) -> u8 {
        self.age
    }
}

impl ShowInfo for People {
    fn new(name: &'static str, age: u8) -> Self {
        People {
            name: name,
            age: age
        }
    }

    fn show_name(&self) {
        println!("Name: {}", self.get_name());
    }

    fn show_age(&self) {
        println!("Age: {}", self.get_age());
    }
}

impl Animal {
    fn noise(&self) {
        println!("Noise: blabla");
    }

    fn talk(&self) {
        println!("Talk: bebe");
    }
}

impl ShowInfo for Animal {
    fn new(name: &'static str, age: u8) -> Self {
        Animal {
            name: name,
            age: age,
        }
    }

    fn show_name(&self) {
        println!("Name: {}", self.name);
    }

    fn show_age(&self) {
        println!("Age: {}", self.age);
    }
}

fn main() {
    let people = People::new("Tan Tan", 18);
    people.show_age();
    people.show_name();
    
    let animal = Animal::new("Dog", 1);
    animal.show_age();
    animal.show_name();
    animal.talk();
    animal.noise();
}
