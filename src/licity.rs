const LEN_CAP_MAX: usize = 17;

const LEN_BITS: usize = 9;
const CAP_BITS: usize = LEN_CAP_MAX - LEN_BITS;

const LEN_MASK: usize = (1 << LEN_BITS) - 1;
const CAP_MASK: usize = (1 << CAP_BITS) - 1;

/// Length and capacity encoded into 17 bits.
#[repr(transparent)]
pub struct Licity {
    licity: u32,
}

impl Licity {
    pub const EMPTY: Self = unsafe { Self::from_raw(0) };

    /// Construct a licity.
    #[inline]
    pub const fn new(len: usize, capacity: usize) -> Option<Self> {
        if len > LEN_MASK || capacity > CAP_MASK || capacity < len {
            None
        } else {
            Some(unsafe { Self::new_unchecked(len, capacity) })
        }
    }

    /// Construct a licity.
    #[inline]
    pub const unsafe fn new_unchecked(len: usize, capacity: usize) -> Self {
        let capacity = (capacity.saturating_sub(len) & CAP_MASK) as u32;
        let len = (len & LEN_MASK) as u32;
        let licity = len | (capacity << LEN_BITS);

        Self { licity }
    }

    /// Construct a licity from a raw value.
    #[inline]
    pub const unsafe fn from_raw(licity: u32) -> Self {
        Self { licity }
    }

    /// Extract the length.
    #[inline]
    pub const fn len(&self) -> usize {
        self.licity as usize & LEN_MASK
    }

    /// Extract the capacity.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.len() + (self.licity as usize >> LEN_BITS)
    }

    /// Returns the raw licity.
    #[inline]
    pub const fn get(&self) -> u32 {
        self.licity
    }
}
