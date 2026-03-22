// DOES NOT COMPILE

fn main() {
    let b = Box::new(0);
    let b2 = b;
    move_a_box(b);
}

fn move_a_box(b: Box<i32>) {
    println!("{b}");
}
