// Test that const evaluation still works in the presence of trivial bounds.
// In particular, this test ensures that the MIR of array lengths is not
// replaced with `unreachable`, unlike the MIR of functions.

#![feature(trivial_bounds)]

fn ok()
where
    str: Sized,
{
    let _: [(); 1] = [(); std::convert::identity(1)]; // OK
}

fn err()
where
    str: Sized,
{
    let _: [(); 1] = [(); std::convert::identity(2)]; //~ERROR mismatched types
}

fn main() {}
