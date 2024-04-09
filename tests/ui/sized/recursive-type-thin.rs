//@ check-fail

#![feature(ptr_metadata)]

trait A {
    type Assoc: std::ptr::Thin;
}

impl A for () {
    type Assoc = Foo<()>;
    //~^ ERROR overflow evaluating the requirement `<Foo<()> as Pointee>::Metadata == ()`
}
struct Foo<T: A>(T::Assoc);

fn main() {}
