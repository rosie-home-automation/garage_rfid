extern crate uuid;
use uuid::Uuid;

fn test1() {
  println!("UUID {}", Uuid::new_v4().to_string());
}

fn main() {
  test1();
}
