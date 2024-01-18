trait Foo { fn foo(&self) {} }
impl<T> Foo for T {}

trait Bar { fn foo(&self) {} }
impl<T> Bar for T {}

enum E {
    A, B
}

// Make sure casts between thin-pointer <-> fat pointer obey RFC401
fn main() {
    let a: &[i32] = &[1, 2, 3];
    let b: Box<[i32]> = Box::new([1, 2, 3]);
    let p = a as *const [i32];
    let q = a.as_ptr();

    a as usize; //~ ERROR casting
    a as isize; //~ ERROR casting
    a as i16; //~ ERROR casting `&[i32]` as `i16` is invalid
    a as u32; //~ ERROR casting `&[i32]` as `u32` is invalid
    b as usize; //~ ERROR non-primitive cast
    p as usize; //~ ERROR casting

    // #22955
    q as *const [i32]; //~ ERROR cannot cast

    // #21397
    let t: *mut (dyn Foo + 'static) = 0 as *mut _;
    //~^ ERROR cannot cast `usize` to a pointer that is wide
    let mut fail: *const str = 0 as *const str;
    //~^ ERROR cannot cast `usize` to a pointer that is wide
    let mut fail2: *const str = 0isize as *const str;
    //~^ ERROR cannot cast `isize` to a pointer that is wide

    let f: f32 = 1.2;
    let v = core::ptr::null::<u8>();
    let fat_v : *const [u8] = &[];
    let fat_sv : *const [i8] = &[];
    let foo: &dyn Foo = &f;

    let _ = v as &u8; //~ ERROR non-primitive cast
    let _ = v as E; //~ ERROR non-primitive cast
    let _ = v as fn(); //~ ERROR non-primitive cast
    let _ = v as (u32,); //~ ERROR non-primitive cast
    let _ = Some(&v) as *const u8; //~ ERROR non-primitive cast

    let _ = v as f32; //~ ERROR is invalid
    let _ = main as f64; //~ ERROR is invalid
    let _ = &v as usize; //~ ERROR is invalid
    let _ = f as *const u8; //~ ERROR is invalid
    let _ = 3_i32 as bool; //~ ERROR cannot cast
    let _ = E::A as bool; //~ ERROR cannot cast
    let _ = 0x61u32 as char; //~ ERROR can be cast as

    let _ = false as f32; //~ ERROR is invalid
    let _ = E::A as f32; //~ ERROR is invalid
    let _ = 'a' as f32; //~ ERROR is invalid

    let _ = false as *const u8; //~ ERROR is invalid
    let _ = E::A as *const u8; //~ ERROR is invalid
    let _ = 'a' as *const u8; //~ ERROR is invalid

    let _ = 42usize as *const [u8]; //~ ERROR cannot cast `usize` to a pointer that is wide
    let _ = v as *const [u8]; //~ ERROR cannot cast
    let _ = fat_v as *const dyn Foo; //~ ERROR the size for values of type
    let _ = foo as *const str; //~ ERROR is invalid
    let _ = foo as *mut str; //~ ERROR is invalid
    let _ = main as *mut str; //~ ERROR is invalid
    let _ = &f as *mut f32; //~ ERROR is invalid
    let _ = &f as *const f64; //~ ERROR is invalid
    let _ = fat_sv as usize; //~ ERROR is invalid

    let a : *const str = "hello";
    let _ = a as *const dyn Foo; //~ ERROR the size for values of type

    // check no error cascade
    let _ = main.f as *const u32; //~ ERROR no field

    let cf: *const dyn Foo = &0;
    let _ = cf as *const [u16]; //~ ERROR is invalid
    let _ = cf as *const dyn Bar; //~ ERROR is invalid

    // casting principal away is not allowed for now
    let _ = cf as *const dyn Send; //~ ERROR is invalid

    vec![0.0].iter().map(|s| s as f32).collect::<Vec<f32>>(); //~ ERROR is invalid
}

fn foo<T: ?Sized>() {
    let s = 0 as *const T;
    //~^ ERROR cannot cast `usize` to a pointer that may be wide
}

fn illegal_cast<U:?Sized,V:?Sized>(u: *const U) -> *const V
{
    u as *const V //~ ERROR is invalid
}

fn illegal_cast_2<U:?Sized>(u: *const U) -> *const str
{
    u as *const str //~ ERROR is invalid
}
