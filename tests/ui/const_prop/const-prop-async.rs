// edition: 2018

#![crate_type = "lib"]

pub async fn foo() -> i32 {
    let array = [1];
    array[1] //~ ERROR this operation will panic at runtime
}
