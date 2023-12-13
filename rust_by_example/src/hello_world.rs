mod debug;
mod display;
mod formatted_print;
mod formatting;
mod hello;
mod testcase_list;

pub fn main() {
    hello::main();
    formatted_print::main();
    debug::main();
    display::main();
    testcase_list::main();
    formatting::main();
}
