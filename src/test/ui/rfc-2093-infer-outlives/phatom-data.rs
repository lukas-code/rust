// Sanity check that `PhantomData<&'a T>` infers `T: 'a`.

use std::marker::PhantomData;
use std::ptr;

struct InferredBound<'a, T> {
    ptr: *const T,
    phantom: PhantomData<&'a T>,
}

struct RedundantBound<'a, T: 'a> {
    ptr: *const T,
    phantom: PhantomData<&'a T>,
}

struct ManualBound<'a, T: 'a> {
    ptr: *const T,
    phantom: PhantomData<&'a ()>,
}

struct NoBound<'a, T> {
    ptr: *const T,
    phantom: PhantomData<&'a ()>,
}

fn inferred_bound<'a>(_: &'a u8) -> InferredBound<'static, &'a u8> {
    InferredBound {
        ptr: ptr::null(),
        phantom: PhantomData,
    }
}

fn redundant_bound<'a>(_: &'a u8) -> RedundantBound<'static, &'a u8> {
    RedundantBound {
        ptr: ptr::null(),
        phantom: PhantomData,
    }
}

fn manual_bound<'a>(_: &'a u8) -> ManualBound<'static, &'a u8> {
    ManualBound {
        ptr: ptr::null(),
        phantom: PhantomData,
    }
}

fn no_bound<'a>(_: &'a u8) -> NoBound<'static, &'a u8> {
    NoBound {
        ptr: ptr::null(),
        phantom: PhantomData,
    }
}

fn main() {
    let local = 0;
    inferred_bound(&local); //~ ERROR: `local` does not live long enough [E0597]
    redundant_bound(&local); //~ ERROR: `local` does not live long enough [E0597]
    manual_bound(&local); //~ ERROR: `local` does not live long enough [E0597]
    no_bound(&local); // OK
}
