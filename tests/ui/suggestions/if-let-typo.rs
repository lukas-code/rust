fn main() {
    let foo = Some(0);
    let bar = None;
    if Some(x) = foo {} //~ ERROR cannot find value `x` in this scope
    //~^ ERROR mismatched types
    if Some(foo) = bar {} //~ ERROR mismatched types
    if 3 = foo {} //~ ERROR mismatched types
    if Some(3) = foo {} //~ ERROR mismatched types
    //~^ ERROR invalid left-hand side of assignment
    if x = 5 {}  //~ ERROR cannot find value `x` in this scope

    // https://github.com/rust-lang/rust/issues/113354
    let _ = || while Some(_) = None {}; //~ ERROR mismatched types
    const _: () = if Ok(0) = Err(0) {}; //~ ERROR mismatched types
    //~^ ERROR invalid left-hand side of assignment
}
