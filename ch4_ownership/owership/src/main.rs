// DOES NOT COMPILE
// s2 = s // happens if b is true or not
fn main() {
  let s = String::from("hello");
  let s2;
  let b = true;
  if b {
    s2 = s;
  }
  println!("{}", s);
}
