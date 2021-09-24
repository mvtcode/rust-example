struct People {
  name: &'static str,
  age: u8,
}

impl People {
  fn new(name: &'static str, age: u8) -> Self {
    People{
      name: name,
      age: age
    }
  }

  fn set_name(&mut self, name: &'static str) {
    self.name = name;
  }

  fn get_name(&self) -> &str {
    self.name
  }

  fn set_age(&mut self, age: u8) {
    self.age = age;
  }

  fn get_age(&self) -> u8 {
    self.age
  }

  fn show_name(&self) {
    println!("Name: {}", self.name);
  }

  fn show_age(&self) {
    println!("Age: {}", self.age);
  }
}

fn main() {
  let mut people = People::new("zai tan", 19);
  people.show_name();
  people.show_age();

  people.set_name("tan tan");
  people.set_age(22);
  people.show_name();
  people.show_age();

  println!("People name: {}", people.get_name());
  println!("People age: {}", people.get_age());
}
