use std::io;

fn main() {
    // Floats
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Numeric operations
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    let remainder = 43 % 5;

    let explicit_casting = 43 as f64 / 5.1;

    // GG, you are real rust programer now
    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let b = [3; 5];

    let first = a[0];
    let second = a[1];

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
