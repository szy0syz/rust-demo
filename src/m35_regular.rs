extern crate regex;
use regex::Regex;

pub fn run() {
  let re = Regex::new(r"\w{5}").unwrap();
  let text = "dcods  ae22";

  // println!("Found match? {}", re.is_match(text));

  match re.captures(text) {
    // Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
    Some(caps) => println!("Found match: {}", &caps[0]),
    None => println!("Could not find match ...")
  }
}
