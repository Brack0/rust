fn main() {
    // Mutable values must be explicit
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Here is a really const baby, f*ck you TS/JS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // Shadowing can change type
    let spaces = "   ";
    let spaces = spaces.len();

    // Mutable cannot
    let mut spaces = "   ";
    // spaces = spaces.len();     <= expected `&str`, found `usize`
}
