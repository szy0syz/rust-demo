pub fn run() {
  let mut x = 10;

  let xr = &x;

  let dom = &x;
  dom += 1  // === x += 1

  println!("x is {}", dom);
}
