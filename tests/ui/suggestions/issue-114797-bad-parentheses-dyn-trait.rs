//run-rustfix
#![allow(dead_code)]

trait Trait {}

fn assert_send(ptr: *mut dyn Trait) -> *mut dyn (Trait + Send) {
    //~^ ERROR incorrect parentheses around trait bounds
    unsafe { std::mem::transmute(ptr) }
}

fn foo2(_: &dyn (Trait + Send)) {}
//~^ ERROR incorrect parentheses around trait bounds

fn foo3(_: &dyn(Trait + Send)) {}
//~^ ERROR incorrect parentheses around trait bounds

fn main() {}
