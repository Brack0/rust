fn main() {
    for i in 1..=100 {
        println!("{}", to_fizz_buzz(i));
    }
}

fn to_fizz_buzz(number: i8) -> String {
    return match number {
        x if x % 15 == 0 => String::from("FizzBuzz"),
        x if x % 5 == 0 => String::from("Buzz"),
        x if x % 3 == 0 => String::from("Fizz"),
        _ => number.to_string(),
    };
}
