mod hashchain;

use hashchain::HashChain;

pub struct HashTable {
    table: Vec<HashChain>,
    t: u32,
    m: u32,
    hash_function: fn(&[u8; 16]) -> [u8; 16],
    reduction: fn(&[u8; 16]) -> [u8; 16],
}

impl HashTable {
    pub fn new() -> Self {
        unimplemented!()
    }

    fn compute_chain(&self, chain: &HashChain, r: u32) -> [u8; 16] {
        assert!(r > self.t);

        let mut x_j = *chain.starting_point();

        for _ in 0..r {
            let h_j = (self.hash_function)(&x_j);
            x_j = (self.reduction)(&h_j);
        }

        x_j
    }

    fn endpoints(&self) -> Vec<&[u8; 16]> {
        self.table.iter().map(|hc| hc.endpoint()).collect()
    }

    pub fn lookup(&self, h: &[u8; 16]) -> Result<[u8; 16], &'static str> {
        let mut y = (self.reduction)(h);
        let mut steps = 1;

        let endpoints = self.endpoints();

        while steps <= self.t {
            if let Ok(i) = endpoints.binary_search(&&y) {
                return Ok(self.compute_chain(self.table.get(i).unwrap(), self.t - steps));
            }

            let h = (self.hash_function)(&y);
            y = (self.reduction)(&h);
            steps += 1;
        }

        Err("Key not found")
    }
}
