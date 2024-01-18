// run-pass

#![feature(ptr_metadata)]

trait Foo {
    fn foo(&self) {}
}

struct Bar;

impl Foo for Bar {}

fn main() {
    // Test we can turn a fat pointer to array back into a thin pointer.
    let a: *const [i32] = &[1, 2, 3];
    let b = a as *const [i32; 2];
    unsafe {
        assert_eq!(*b, [1, 2]);
    }

    // Test conversion to an address (usize).
    let a: *const [i32; 3] = &[1, 2, 3];
    let b: *const [i32] = a;
    assert_eq!(a as usize, b as *const () as usize);

    // Casting to a different type keeps the metadata.
    let c = b as *const [u8];
    unsafe {
        assert_eq!((*a).len(), (*c).len());
    }

    // `str` <-> `[T]` conversions are allowed.
    let a: *const [u32] = &[1953723730, 544434464, 2053205345, 560426601];
    let b = a as *const str;
    unsafe {
        if cfg!(target_endian = "little") {
            assert_eq!((*b), *"Rust");
        } else if cfg!(target_endian = "big") {
            assert_eq!((*b), *"tsuR");
        }
    }
    let a: *const str = "hello";
    let b = a as *const [u8];
    unsafe {
        assert_eq!((*b), [104, 101, 108, 108, 111]);
    }

    // And conversion to a void pointer/address for trait objects too.
    let a: *mut dyn Foo = &mut Bar;
    let b = a as *mut () as usize;
    let c = a as *const () as usize;
    let d = a.to_raw_parts().0 as usize;

    assert_eq!(b, d);
    assert_eq!(c, d);

    // Adding auto traits is OK.
    let _ = a as *mut (dyn Foo + Send);

    // Casting between auto-trait-only trait objects is OK.
    let unprincipled: *mut dyn Send = &mut Bar;
    let _ = unprincipled as *mut dyn Sync;
}
