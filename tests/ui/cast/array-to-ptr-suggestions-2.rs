fn main() {
    let mut a: [u8; 3] = [1, 2, 3];

    let _ = &a as *const i8;
    //~^ ERROR casting `&[u8; 3]` as `*const i8` is invalid
    //~| HELP you can cast to `*const [u8; 3]` or `*const u8` instead
    //~| HELP cast to `*const [u8; 3]` instead
    //~| HELP cast to `*const u8` instead

    let _ = &mut a as *mut i8;
    //~^ ERROR casting `&mut [u8; 3]` as `*mut i8` is invalid
    //~| HELP you can cast to `*mut [u8; 3]` or `*mut u8` instead
    //~| HELP cast to `*mut [u8; 3]` instead
    //~| HELP cast to `*mut u8` instead

    let _ = &mut a as *const i8;
    //~^ ERROR casting `&mut [u8; 3]` as `*const i8` is invalid
    //~| HELP you can cast to `*const [u8; 3]` or `*const u8` instead
    //~| HELP cast to `*const [u8; 3]` instead
    //~| HELP cast to `*const u8` instead

    // don't suggest casting to `*mut X` here:
    let _ = &a as *mut i8;
    //~^ ERROR casting `&[u8; 3]` as `*mut i8` is invalid
}
