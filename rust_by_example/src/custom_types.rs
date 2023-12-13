mod c_like;
mod constants;
mod enums;
mod structures;
mod testcase_linked_list;
mod use_keyword;

pub fn main() {
    structures::main();
    enums::main();
    use_keyword::main();
    c_like::main();
    testcase_linked_list::main();
    constants::main();
}
