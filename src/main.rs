use std::io;

fn main() {
  let mut name = String::new();

  println!("Please enter your name: ");

  match io::stdin().read_line(&mut name) {
    Ok(..) => 0,
    Err(..) => 1
  };

  println!("Hello {}", name);
}
