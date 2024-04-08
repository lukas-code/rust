//@check-pass

trait Foo {}
impl<'a, T> Foo for &'a T {}

struct Ctx<'a>(&'a ())
where
    &'a (): Foo,
    &'static (): Foo;

fn main() {}
