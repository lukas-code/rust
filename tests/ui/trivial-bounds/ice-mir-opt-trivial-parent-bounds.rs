//@ compile-flags: -C opt-level=1 --crate-type lib -Z validate-mir
//@ edition: 2018
//@ check-pass

// This used to ICE during MIR optimization.

#![feature(trivial_bounds)]

// from https://github.com/rust-lang/rust/issues/135128
pub async fn return_str() -> str
where
    str: Sized, //~ WARNING trivial_bounds
{
    *"Sized".to_string().into_boxed_str()
}


pub fn return_str2() -> impl Sized
where
    str: Sized, //~ WARNING trivial_bounds
{
    || { *"Sized".to_string().into_boxed_str() }
}

// from https://github.com/rust-lang/rust/issues/121363
pub struct TwoStrs(str, str)
where
    str: Sized; //~ WARNING trivial_bounds

impl ::std::fmt::Debug for TwoStrs
where
    str: Sized, //~ WARNING trivial_bounds
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_tuple("TwoStrs")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
