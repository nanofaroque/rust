// ignore-musl - dlsym doesn't see symbols without "-C link-arg=-Wl,--export-dynamic"

#![feature(rustc_private)]

extern crate rustc_metadata;

use rustc_metadata::dynamic_lib::DynamicLibrary;

#[no_mangle]
pub fn foo() {
    bar();
}

pub fn foo2<T>() {
    fn bar2() {
        bar();
    }
    bar2();
}

#[no_mangle]
fn bar() {}

#[allow(dead_code)]
#[no_mangle]
fn baz() {}

pub fn test() {
    let lib = DynamicLibrary::open(None).unwrap();
    unsafe {
        assert!(lib.symbol::<isize>("foo").is_ok());
        assert!(lib.symbol::<isize>("baz").is_ok());
        assert!(lib.symbol::<isize>("bar").is_ok());
    }
}
