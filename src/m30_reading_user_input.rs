use std::io;

pub fn run() {
  let mut input = String::new();

  println!("Hey mate! Say something:");

  match io::stdin().read_line(&mut input) {
    Ok(_) => {
      println!("Success! You said: {}", input.to_uppercase());
    },
    Err(e) => println!("Oops! Something went wrong: {}", e)
  }
}
