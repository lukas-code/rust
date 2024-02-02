#![feature(ptr_metadata, ptr_metadata_cast)]

use std::ptr::{DynMetadata, MetadataCast, PointerCast};

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

trait Trait {}
trait Trait2 {}

fn main() {
    check::<(), usize>(); //~ ERROR not satisfied
    check::<(), DynMetadata<dyn Trait>>(); //~ ERROR not satisfied
    check::<usize, DynMetadata<dyn Trait>>(); //~ ERROR not satisfied
    check::<DynMetadata<dyn Trait>, usize>(); //~ ERROR not satisfied
    check::<DynMetadata<dyn Trait>, dyn Trait + Send>(); //~ ERROR not satisfied
    check::<DynMetadata<dyn Trait>, DynMetadata<dyn Trait2>>(); //~ ERROR not satisfied
    check::<dyn Trait + Send, dyn Trait + Sync>(); //~ ERROR not satisfied

    // `dyn MetadataCast<usize>` should not implement `MetadataCast<usize>`
    check::<dyn MetadataCast<usize>, usize>(); //~ ERROR not satisfied

    // this could pass in the future, but for now it matches the behavior of `as` casts
    check::<DynMetadata<dyn Trait>, DynMetadata<dyn Send>>(); //~ ERROR not satisfied
}

fn do_casts<'a>(thin: &mut i32, slice: &mut [i32], trait_object: &mut (dyn Trait + Send)) {
    let _: *mut [u8] = cast(thin); //~ ERROR not satisfied
    let _: *mut dyn Trait = cast(thin); //~ ERROR not satisfied
    let _: *mut [u8] = cast(trait_object); //~ ERROR not satisfied
    let _: *mut dyn Trait = cast(slice); //~ ERROR not satisfied
    let _: *mut dyn Trait2 = cast(trait_object); //~ ERROR not satisfied

    // may be allowed in the future
    let _: *mut dyn Send = cast(trait_object); //~ ERROR not satisfied
}
