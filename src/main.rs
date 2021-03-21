use strtok::strtok;

fn main() {
  let mut hello_world = "Hello world!";

  println!("Before call to strtok");
  println!("hello_world = {}", hello_world);

  let hello = strtok(&mut hello_world, ' ');

  println!("After call to strtok");
  println!("hello_world = {}", hello_world);
  println!("hello = {}", hello);

  assert_eq!(hello_world, "world!");
  assert_eq!(hello, "Hello");
}
