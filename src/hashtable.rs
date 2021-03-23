use rand::prelude::*;
use rand_pcg::Pcg64;

mod hashchain;

pub use hashchain::HashChain;

const TEXT_LENGTH: usize = 4;
const HASH_LENGTH: usize = 16;

pub type Text = [u8; TEXT_LENGTH];
pub type Hash = [u8; HASH_LENGTH];

#[allow(unused_attributes)]
pub struct HashTable {
    table: Vec<HashChain>,
    t: u32,
    m: u32,
    hash_function: fn(&Text) -> Hash,
    reduction: fn(&Hash) -> Text,
}

impl HashTable {
    pub fn new(
        t: u32,
        m: u32,
        hash_function: fn(&Text) -> Hash,
        reduction: fn(&Hash) -> Text,
    ) -> Self {
        let mut table = Vec::new();

        let sps: Vec<Text>;

        let mut rng = Pcg64::seed_from_u64(2);

        sps = (0..m)
            .map(|_| {
                let mut sp = [0u8; TEXT_LENGTH];
                rng.fill(&mut sp);
                sp
            })
            .collect();

        for sp in sps {
            let mut x_j = sp;

            for _ in 0..t {
                let h_j = hash_function(&x_j);
                x_j = reduction(&h_j);
            }

            table.push(HashChain::new(sp, x_j));
        }

        table.sort_unstable();

        HashTable {
            table,
            t,
            m,
            hash_function,
            reduction,
        }
    }

    fn compute_chain(&self, chain: &HashChain, r: u32) -> Text {
        assert!(r < self.t);

        let mut x_j = *chain.starting_point();

        for _ in 0..r {
            let h_j = (self.hash_function)(&x_j);
            x_j = (self.reduction)(&h_j);
        }

        x_j
    }

    fn endpoints(&self) -> Vec<&Text> {
        self.table.iter().map(|hc| hc.endpoint()).collect()
    }

    pub fn lookup(&self, h: &Hash) -> Result<Text, &'static str> {
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
