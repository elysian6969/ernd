#![deny(warnings)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(const_pointer_byte_offsets)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_trait_impl)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_const_cast)]
#![feature(strict_provenance)]

pub use tagged::Tagged;
pub use tiny_slice::TinySlice;
pub use tiny_str::TinyStr;

mod tagged;
mod tiny_slice;
mod tiny_str;

/*#[derive(Eq, PartialEq)]
pub enum StorageKind {
    Word,
    DoubleWord,
    TripleWord,
}

pub struct Backend<const STORAGE: StorageKind>;

pub trait Storage<T>
where
    T: ?Sized,
{
    type Data;

    fn borrowed(value: &T) -> Self::Data:
}

impl const Storage<str> for Backend<{ StorageKind::Word }> {
    type Data = Tagged<u8>;

    fn borrowed(value: &str) -> Self::Data {
        let ptr = value.as_ptr();
        let len = value.len();
        let data = unsafe {
            Tagged::new_unchecked(ptr.as_mut()).with_tag(len)
        };

        data
    }
}

pub struct Cow<'a, T, const STORAGE: StorageKind>
where
    T: ?Sized + 'a,
    Backend<{ STORAGE }>: Storage<T>,
{
    pub data: <Backend<{ STORAGE }> as Storage<T>>::Data,
    _phantom: PhantomData<&'a T>,
}

impl Cow<'a, T, const STORAGE: StorageKind>
where
    T: ?Sized + 'a,
    Backend<{ STORAGE }>: Storage<T>,
{
    #[inline]
    pub const fn borrowed(value: T) ->Self {
        let data = <Backend<{ STORAGE }> as Storage<T>>::borrowed(value);

        Self { data, _phantom: PhantomData }
    }
}*/
