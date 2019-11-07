pub fn fun() {
  let mut x 10;

  {
    let x = 15;
  }

  let x = "X is a starting";
  println!("x is {}", x); // -> X is a starting

  let x = true;
  println!("x is {}", x); // -> true
}
