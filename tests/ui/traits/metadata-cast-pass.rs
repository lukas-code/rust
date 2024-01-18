// check-pass
#![feature(ptr_metadata, ptr_metadata_cast)]

use std::ptr::{self, DynMetadata, MetadataCast, Pointee, PointerCast, Thin};

fn cast<T: ?Sized, U: ?Sized>(_: *mut T) -> *mut U
where
    T: PointerCast<U>,
{
    todo!()
}

fn check<T: ?Sized, U: ?Sized>()
where
    T: MetadataCast<U>,
{
}

trait Trait {}

fn main() {
    // any to () is OK
    check::<(), ()>();
    check::<usize, ()>();
    check::<DynMetadata<dyn Trait>, ()>();
    check::<dyn MetadataCast<usize>, ()>();

    // same types are OK
    check::<usize, usize>();
    check::<i32, i32>();
    check::<DynMetadata<dyn Trait>, DynMetadata<dyn Trait>>();
    check::<dyn MetadataCast<usize>, dyn MetadataCast<usize>>();

    // changing auto traits of trait object in DynMetadata is OK
    check::<DynMetadata<dyn Send>, DynMetadata<dyn Sync>>();
    check::<DynMetadata<dyn Trait + Send>, DynMetadata<dyn Trait + Sync>>();
}

fn lifetimes_ok<'a, 'b>() {
    // changing lifetimes of trait object in DynMetadata is OK
    check::<DynMetadata<dyn Trait + 'a>, DynMetadata<dyn Trait + 'b>>();

    // otherwise, require source outlives target
    check::<&'a (), &'a ()>();
    check::<&'static (), &'a ()>();
}

fn do_casts<'a>(thin: &mut i32, slice: &mut [i32], trait_object: &mut dyn Trait) {
    let _: *mut u8 = cast(thin);
    let _: *mut u8 = cast(slice);
    let _: *mut [u8] = cast(slice);
    let _: *mut u8 = cast(trait_object);
    let _: *mut (dyn Trait + Send + 'a) = cast(trait_object);
}

fn subtype_ok<'short, 'long: 'short, T, U>(ptr: *mut T) -> *mut U
where
    T: ?Sized + Pointee<Metadata = &'long ()>,
    U: ?Sized + Pointee<Metadata = &'short ()>,
{
    if true {
        // Because this allows subtyping ...
        ptr::from_raw_parts_mut(ptr.cast::<()>(), ptr::metadata(ptr))
    } else {
        // ... `PointerCast` should allow it as well
        cast(ptr)
    }
}

fn same_metadata<T: ?Sized, U: ?Sized>(ptr: *mut T) -> *mut U
where
    T: Pointee<Metadata = <U as Pointee>::Metadata>,
{
    cast(ptr)
}

fn to_thin<T: ?Sized, U: ?Sized + Thin>(ptr: *mut T) -> *mut U {
    cast(ptr)
}
