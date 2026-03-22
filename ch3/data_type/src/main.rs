use std::io;
fn main() {
    let mut x: (i32, f32, bool) = (-3, 4.5, true);
    x.1 += 5.0;
    println!("{0}",x.1);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
