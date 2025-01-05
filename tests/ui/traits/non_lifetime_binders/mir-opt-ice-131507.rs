//@ compile-flags: -Zmir-enable-passes=+GVN -Zmir-enable-passes=+Inline -Zvalidate-mir
//@ check-pass

#![feature(non_lifetime_binders)]
#![expect(incomplete_features)]

fn brick()
where
    for<T> T: Copy,
{
    || format_args!("");
}

fn main() {}
