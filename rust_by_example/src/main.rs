mod conversion;
mod custom_types;
mod expressions;
mod hello_world;
mod primitives;
mod types;
mod variable_bindings;

fn main() {
    println!("Hello, world!");

    hello_world::main();
    primitives::main();
    custom_types::main();
    variable_bindings::main();
    types::main();
    conversion::main();
    expressions::main();
}
