// run-pass

#![feature(offset_of)]
#![allow(dead_code)]

use std::mem::offset_of;

#[derive(Clone)]
struct WideptrField<T: ?Sized> {
    first: usize,
    second: usize,
    niche: NicheAtEnd,
    tail: T,
}

#[derive(Clone)]
#[repr(C)]
struct NicheAtEnd {
    arr: [u8; 7],
    b: bool,
}

type Tail = [bool; 8];

fn main() {
    let sized = Box::new(WideptrField {
        first: 0xdead,
        second: 0xbeef,
        niche: NicheAtEnd {
            arr: [0; 7],
            b: false,
        },
        tail: [false; 8] as Tail,
    });

    let _unsized_value: Box<WideptrField<dyn Send>> = sized.clone();

    assert_eq!(
        offset_of!(WideptrField<Tail>, niche),
        offset_of!(WideptrField<dyn Send>, niche),
    );
}
