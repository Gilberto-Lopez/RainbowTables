use std::cmp::Ordering;

use super::Text;

// Hashchain with starting point sp and endpoint ep
pub struct HashChain {
    sp: Text,
    ep: Text,
}

impl Ord for HashChain {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ep.cmp(&other.ep)
    }
}

impl PartialOrd for HashChain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ep.partial_cmp(&other.ep)
    }
}

impl PartialEq for HashChain {
    fn eq(&self, other: &Self) -> bool {
        self.ep == other.ep
    }
}

impl Eq for HashChain {}

impl HashChain {
    pub fn new(sp: Text, ep: Text) -> Self {
        HashChain { sp, ep }
    }

    pub fn starting_point(&self) -> &Text {
        &self.sp
    }

    pub fn endpoint(&self) -> &Text {
        &self.ep
    }
}
