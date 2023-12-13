mod declare_first;
mod freezing;
mod mutability;
mod scope_and_shadowing;
mod variable_bindings;

pub fn main() {
    variable_bindings::main();
    mutability::main();
    scope_and_shadowing::main();
    declare_first::main();
    freezing::main();
}
