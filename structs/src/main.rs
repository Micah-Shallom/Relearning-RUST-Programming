#![allow(dead_code)]
mod method;
mod structs;
mod tuple_struct;

fn main() {
    structs::structfn();
    tuple_struct::tuplestructsfn();
    tuple_struct::example_program();
    method::methodfn();
}
