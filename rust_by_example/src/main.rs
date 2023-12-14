mod conversion;
mod custom_types;
mod expressions;
mod flow_of_control;
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
    flow_of_control::main();
}
