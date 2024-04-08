trait Foo {
    type Context<'c>
    where
        Self: 'c;
}

impl Foo for Box<dyn Foo> {}
//~^ ERROR cycle detected
//~| ERROR cycle detected
//~| ERROR cycle detected
//~| ERROR the trait `Foo` cannot be made into an object
//~| ERROR the trait bound `Box<(dyn Foo + 'static)>: Foo` is not satisfied
//~| ERROR the trait `Foo` cannot be made into an object
//~| ERROR not all trait items implemented, missing: `Context`

fn main() {}
