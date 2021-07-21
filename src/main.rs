use myrust::*;
use std::borrow::Borrow;

mod common;
mod args;
mod concurrent;
mod duotai;

fn main() {
    lib_print();
    concurrent::multi_thread4();
}