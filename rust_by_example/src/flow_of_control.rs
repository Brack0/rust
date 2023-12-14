mod for_loop;
mod if_else;
mod if_let;
mod let_else;
mod loop_keyword;
mod match_arrays_slices;
mod match_binding;
mod match_enums;
mod match_guards;
mod match_keyword;
mod match_pointers_ref;
mod match_structs;
mod match_tuples;
mod nesting_and_labels;
mod returning_from_loops;
mod while_keyword;
mod while_let;

pub fn main() {
    if_else::main();
    loop_keyword::main();
    nesting_and_labels::main();
    returning_from_loops::main();
    while_keyword::main();
    for_loop::main();
    match_keyword::main();
    match_tuples::main();
    match_arrays_slices::main();
    match_enums::main();
    match_pointers_ref::main();
    match_structs::main();
    match_guards::main();
    match_binding::main();
    if_let::main();
    let_else::main();
    while_let::main();
}
