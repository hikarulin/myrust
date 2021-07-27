use myrust::*;
use std::borrow::Borrow;

mod common;
mod args;
mod concurrent;
mod duotai;
mod http;

fn main() {
    // lib_print();
    // concurrent::modify_value_multi_thread();
    http::http_listen();
}