trait LifetimeParam<'a> {}
fn lifetime_param<'a, 'b>(ptr: *const dyn LifetimeParam<'a>) -> *const dyn LifetimeParam<'b> {
    ptr as _
    //~^ ERROR lifetime may not live long enough
    //~| ERROR lifetime may not live long enough
}

fn main() {}
