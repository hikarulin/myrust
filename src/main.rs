use myrust::*;
use std::borrow::Borrow;

mod common;
mod args;
mod concurrent;
mod duotai;

fn main() {
    lib_print();
    concurrent::modify_value_multi_thread();
}