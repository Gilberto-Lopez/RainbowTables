use crate::hashtable::{Hash, HashChain, Text};

pub struct RainbowTable {
    table: Vec<HashChain>,
    t: u32,
    m: u32,
    hash_function: fn(&Text) -> Hash,
    reductions: Vec<fn(&Hash) -> Text>,
}

impl RainbowTable {
    fn compute_chain(&self, chain: &HashChain, r: u32) -> Text {
        assert!(r <= self.t);

        let mut x_j = *chain.starting_point();

        let mut iter = self.reductions.iter();

        for _ in 0..r {
            let R = iter.next().unwrap();
            let h_j = (self.hash_function)(&x_j);
            x_j = (R)(&h_j);
        }

        x_j
    }

    fn endpoints(&self) -> Vec<&Text> {
        self.table.iter().map(|hc| hc.endpoint()).collect()
    }

    pub fn lookup(&self, h: &Hash) -> Result<Text, &'static str> {
        let mut steps = 1;
        let endpoints = self.endpoints();

        while steps <= self.t {
            let j = (self.t - steps) as usize;
            let rs = &self.reductions[j..];

            let mut iter = rs.iter();

            let mut y = (iter.next().unwrap())(h);

            for R in iter {
                let h = (self.hash_function)(&y);
                y = (R)(&h);
            }

            if let Ok(i) = endpoints.binary_search(&&y) {
                return Ok(self.compute_chain(self.table.get(i).unwrap(), self.t - steps));
            }

            steps += 1;
        }

        Err("Key not found")
    }
}
