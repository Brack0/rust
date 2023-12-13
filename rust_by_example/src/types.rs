mod aliasing;
mod casting;
mod inference;
mod literals;

pub fn main() {
    casting::main();
    literals::main();
    inference::main();
    aliasing::main();
}
