use myrust::*;

mod common;
mod args;
mod concurrent;

fn main() {
    lib_print();
    concurrent::multi_thread4();
}