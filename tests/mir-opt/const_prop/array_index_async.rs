// unit-test: ConstProp
// edition: 2018
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// EMIT_MIR_FOR_EACH_BIT_WIDTH

#![crate_type = "lib"]

// EMIT_MIR array_index_async.main-{closure#0}.ConstProp.diff
pub async fn main() {
    let x: u32 = [0, 1, 2, 3][2];
}
