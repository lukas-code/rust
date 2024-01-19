#![feature(ptr_metadata, ptr_metadata_cast)]

use std::ptr::{self, DynMetadata, MetadataCast, Pointee, PointerCast};

fn cast<T: ?Sized, U: ?Sized>(ptr: *mut T) -> *mut U
where
    T: PointerCast<U>,
{
    ptr as _
}

fn check<T: ?Sized, U: ?Sized>()
where
    T: MetadataCast<U>,
{
}

fn main() {}

fn err1<'a, 'b>() {
    // changing lifetimes in `DynMetadata<T>` is only allowed if `T = dyn Trait`
    check::<DynMetadata<&'a ()>, DynMetadata<&'b ()>>();
    //~^ ERROR may not live long enough
}

fn err2<'a, 'b>() {
    check::<&'a (), &'b ()>();
    //~^ ERROR may not live long enough
}

fn subtype_fail<'short, 'long: 'short, T, U>(ptr: *mut T) -> *mut U
where
    T: ?Sized + Pointee<Metadata = &'short ()>,
    U: ?Sized + Pointee<Metadata = &'long ()>,
{
    // extending the lifetime of arbitrary metadata is not allowed
    cast(ptr)
    //~^ ERROR may not live long enough
}
