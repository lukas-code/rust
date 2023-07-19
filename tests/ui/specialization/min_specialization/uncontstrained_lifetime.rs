// issue: 103708
// issue: 113045

// Test that we don't ICE due to unconstrained lifetimes.

#![feature(min_specialization)]

trait Dance {}

impl<'a, T> Dance for T {}

impl Dance for bool {}
//~^ ERROR specialization impl does not specialize
//~| ERROR could not resolve substs on overridden impl

trait X {}

impl<'a, const N: usize> X for [(); N] {}

impl<'a, Unconstrained> X for [(); 0] {}
//~^ ERROR the type parameter `Unconstrained` is not constrained
//~| ERROR specialization impl does not specialize
//~| ERROR could not resolve substs on overridden impl

fn main() {}
