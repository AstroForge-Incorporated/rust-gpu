// HACK(eddyb) duplicate of index_user_dst.spirt.rs because only-/ignore- do not work with revisions.
// only-not_spirt
#![crate_name = "index_user_dst"]

// build-pass
// compile-flags: -C llvm-args=--disassemble-entry=main

use spirv_std::spirv;

#[spirv(fragment)]
pub fn main(#[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slice: &mut [f32]) {
    let float: f32 = slice[0];
    let _ = float;
}
