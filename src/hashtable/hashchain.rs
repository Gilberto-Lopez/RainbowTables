use std::cmp::Ordering;
use std::fmt;

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

impl fmt::LowerHex for HashChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sp_rep: Vec<String> = self.sp.iter().map(|b| format!("{:02x}", b)).collect();
        let ep_rep: Vec<String> = self.ep.iter().map(|b| format!("{:02x}", b)).collect();

        let sp_hex = sp_rep.join("");
        let ep_hex = ep_rep.join("");

        write!(f, "{},{}", sp_hex, ep_hex)
    }
}

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
