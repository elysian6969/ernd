use core::mem;
use ernd::{Cow, StorageKind, TinySlice, TinyStr};

fn main() {
    let slice = TinySlice::from_slice(&[1, 2, 3]);

    println!("slice = {slice:?}");
    println!("size  = {:?}", mem::size_of::<TinySlice<'_, i32>>());

    let string = TinyStr::from_str("hello world");

    println!("string = {string:?}");
    println!("size   = {:?}", mem::size_of::<TinyStr<'_>>());

    let cow: Cow<'_, str, { StorageKind::Tiny }> = Cow::borrowed("xd");

    println!("cow  = {cow:?}");
    println!(
        "size = {:?}",
        mem::size_of::<Cow<'_, str, { StorageKind::Tiny }>>()
    );
}
