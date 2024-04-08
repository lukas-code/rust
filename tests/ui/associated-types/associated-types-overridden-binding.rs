//@revisions: pass fail
//@[pass] check-pass

#![feature(trait_alias)]

trait Foo: Iterator<Item = i32> {}
trait Bar: Foo<Item = u32> {}

trait I32Iterator = Iterator<Item = i32>;
trait U32Iterator = I32Iterator<Item = u32>;

#[cfg(fail)]
fn normalize_bar<T: Bar>(_: T::Item) {}
//[fail]~^ ERROR type annotations needed
//[fail]~| ERROR type annotations needed

#[cfg(fail)]
fn normalize_u32iterator<T: U32Iterator>(_: T::Item) {}
//[fail]~^ ERROR type annotations needed
//[fail]~| ERROR type annotations needed

fn main() {
    let _: &dyn Foo<Item = u32>; // OK
    let _: &dyn I32Iterator<Item = u32>; // OK
}
