// check-pass

#![feature(ptr_metadata)]

use std::ptr::{self, Pointee};

fn cast_same_meta<T: ?Sized, U: ?Sized>(ptr: *const T) -> *const U
where
    T: Pointee<Metadata = <U as Pointee>::Metadata>,
{
    let (thin, meta) = ptr.to_raw_parts();
    ptr::from_raw_parts(thin, meta)
}

struct Wrapper<T: ?Sized>(T);

// if `Wrapper<T>: ?Sized` then normalize `Wrapper<T>::Metadata` -> `T::Metadata`
fn wrapper_to_tail<T: ?Sized>(ptr: *const T) -> *const Wrapper<T> {
    cast_same_meta(ptr)
}

// if `Wrapper<T>: Sized` then normalize `Wrapper<T>::Metadata` -> `()`
fn wrapper_to_unit<T: ?Sized>(ptr: *const ()) -> *const Wrapper<T>
where
    Wrapper<T>: Sized,
{
    cast_same_meta(ptr)
}

trait Project {
    type Assoc: ?Sized;
}

struct WrapperProject<T: ?Sized + Project>(T::Assoc);

// normalize `WrapperProject<T>::Metadata` -> `T::Assoc::Metadata`
fn wrapper_project<T: ?Sized + Project>(ptr: *const T::Assoc) -> *const WrapperProject<T> {
    cast_same_meta(ptr)
}

// In this example, `WrapperProject<&'a T>` is `Sized` modulo regions,
// but `?Sized` considering regions.
// Normalize `WrapperProject<&'a T>::Metadata` -> `<&'a T as Project>::Assoc::Metadata`
// and don't require `WrapperProject<&'a T>: Sized`.
fn sized_modulo_regions<'a, T: ?Sized + 'static>(
    ptr: *const <&'a T as Project>::Assoc,
) -> *const WrapperProject<&'a T>
where
    for<'b> &'b T: Project,
    WrapperProject<&'static T>: Sized,
{
    cast_same_meta(ptr)
}

fn main() {}
