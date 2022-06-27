pub const BITS: usize = 64;

pub const PTR_BITS: usize = BITS - TAG_BITS;
pub const TAG_BITS: usize = 17;

pub const PTR_MASK: usize = (1 << PTR_BITS) - 1;
pub const TAG_MASK: usize = (1 << TAG_BITS) - 1;

pub const LEN_CAP_MAX: usize = 17;

pub const LEN_BITS: usize = 9;
pub const CAP_BITS: usize = LEN_CAP_MAX - LEN_BITS;

pub const LEN_MASK: usize = (1 << LEN_BITS) - 1;
pub const CAP_MASK: usize = (1 << CAP_BITS) - 1;
