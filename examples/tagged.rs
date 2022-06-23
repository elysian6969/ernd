use core::mem;
use fun::{TinySlice, TinyStr};

fn main() {
    let slice = TinySlice::from_slice(&[1, 2, 3]);

    println!("slice = {slice:?}");
    println!("size  = {:?}", mem::size_of::<TinySlice<'_, i32>>());

    let string = TinyStr::from_str("hello world");

    println!("string = {string:?}");
    println!("size   = {:?}", mem::size_of::<TinyStr<'_>>());
}
