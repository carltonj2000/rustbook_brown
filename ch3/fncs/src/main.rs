fn main() {
    println!("Hello, world!");

    another_function(4, 'c');

    let y = {
        let x = 7;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = some_number();
    
    println!("The value of x is: {x}");

    let z = plus_one(x);
    
    println!("The value of z is: {z}");

}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}

fn some_number() -> i32 {
    5
}

fn plus_one(n: i32) -> i32 {
    n + 1
}
