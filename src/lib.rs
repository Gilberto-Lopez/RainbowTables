pub mod hashtable;
pub mod rainbowtable;

pub const TEXT_LENGTH: usize = 4;
pub const HASH_LENGTH: usize = 16;

pub type Text = [u8; TEXT_LENGTH];
pub type Hash = [u8; HASH_LENGTH];
