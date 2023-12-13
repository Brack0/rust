pub fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to "y"
        x_cube + x_squared + x
    };

    #[allow(unused_must_use)]
    let z = {
        // The semicolon suppresses this expression and "()" is assigned to "z"
        2 * x; // unused_must_use lint rule is triggered here
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
