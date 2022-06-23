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

use core::fmt;
use core::mem::ManuallyDrop;
use core::marker::PhantomData;

/// Storage variant.
#[derive(Eq, PartialEq)]
pub enum StorageKind {
    Tiny,
}

pub struct Backend<const STORAGE: StorageKind>;

union TinyStorageUnion<'a> {
    borrowed: ManuallyDrop<TinyStr<'a>>,
    owned: ManuallyDrop<String>,
}

pub struct TinyStorage<'a> {
    data: TinyStorageUnion<'a>,
}

impl<'a> TinyStorage<'a> {
    #[inline]
    pub const fn borrowed(string: &'a str) -> Self {
        TinyStorage {
            data: TinyStorageUnion {
                borrowed: ManuallyDrop::new(TinyStr::from_str(string)),
            },
        }
    }

    #[inline]
    pub const fn owned(string: String) -> Self {
        TinyStorage {
            data: TinyStorageUnion {
                owned: ManuallyDrop::new(string),
            },
        }
    }
}

/// encodes both length and capacity
#[repr(transparent)]
pub struct LenCap {
    len_cap: usize,
}

impl LenCap {
    pub const fn new(len: usize, cap: usize) -> Self {
        // 8 bits for capacity
        let cap = cap.saturating_sub(len) & ((1 << 8) - 1);

        // 9 bits for length
        let len = len & ((1 << 9) - 1);

        Self { len_cap: len | (cap << 9) }
    }

    pub const fn len(&self) -> usize {
        self.len_cap & ((1 << 9) - 1)
    }

    pub const fn capacity(&self) -> usize {
        self.len() + (self.len_cap & ((1 << 8) - 1))
    }
}

impl<'a> fmt::Debug for TinyStorage<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(
    }
}

/// Implementation for each storage variant.
pub trait Storage<'a, T>
where
    T: ?Sized + 'a,
{
    type Data;
    type Owned;

    fn borrowed(borrowed: &'a T) -> Self::Data;
    fn owned(owned: Self::Owned) -> Self::Data;
}

impl<'a> const Storage<'a, str> for Backend<{ StorageKind::Tiny }> {
    type Data = TinyStorage<'a>;
    type Owned = String;

    fn borrowed(string: &'a str) -> Self::Data {
        TinyStorage::borrowed(string)
    }

    fn owned(string: String) -> Self::Data {
        TinyStorage::owned(string)
    }
}

#[allow(dead_code)]
pub struct Cow<'a, T, const STORAGE: StorageKind>
where
    T: ?Sized + 'a,
    Backend<{ STORAGE }>: Storage<'a, T>,
{
    data: <Backend<{ STORAGE }> as Storage<'a, T>>::Data,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T, const STORAGE: StorageKind> Cow<'a, T, STORAGE>
where
    T: ?Sized + 'a,
    Backend<{ STORAGE }>: Storage<'a, T>,
{
    #[inline]
    pub const fn borrowed(value: &'a T) -> Self
    where
        Backend<{ STORAGE }>: ~const Storage<'a, T>,
    {
        let data = <Backend<{ STORAGE }> as Storage<'a, T>>::borrowed(value);

        Self {
            data,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, const STORAGE: StorageKind> fmt::Debug for Cow<'a, T, STORAGE>
where
    T: fmt::Debug + ?Sized + 'a,
    Backend<{ STORAGE }>: Storage<'a, T>,
    <Backend<{ STORAGE }> as Storage<'a, T>>::Data: fmt::Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.data, fmt)
    }
}
