use std::cmp::Ordering;

// Hashchain with starting point sp and endpoint ep
pub struct HashChain {
    sp: [u8; 16],
    ep: [u8; 16],
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
