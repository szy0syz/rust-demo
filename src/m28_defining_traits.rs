struct Person {
  name: String,
  age: u8
}

trait HasVoiceBox {
  // Speak
  fn speak(&self);

  //Check if can speak
  fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
  fn speak(&self) {
    println!("Hello, my name is {}", self.name);
  }

  fn can_speak(&self) -> bool {
    if self.age > 0 {
      return true;
    } return false;
  }
}

pub fn run () {
  let person = Person {
    name: String::from("Jerry"),
    age: 0
  };

  println!("Can {} spean? {}", person.name, person.can_speak());
}
