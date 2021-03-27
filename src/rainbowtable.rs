use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::hashtable::HashChain;
use crate::{Hash, Text, TEXT_LENGTH};

#[allow(dead_code)]
pub struct RainbowTable {
    table: Vec<HashChain>,
    t: u32,
    m: u32,
    hash_function: fn(&Text) -> Hash,
    reductions: Vec<fn(&Hash) -> Text>,
}

#[allow(non_snake_case)]
impl RainbowTable {
    pub fn new(
        t: u32,
        m: u32,
        hash_function: fn(&Text) -> Hash,
        reductions: Vec<fn(&Hash) -> Text>,
    ) -> Self {
        assert!(t == reductions.len() as u32);

        let mut table: Vec<HashChain>;

        let mut rng = Pcg64::seed_from_u64(2);

        table = (0..m)
            .map(|_| {
                let mut sp = [0u8; TEXT_LENGTH];
                rng.fill(&mut sp);

                let mut x_j = sp;

                for R in &reductions {
                    let h_j = hash_function(&x_j);
                    x_j = (R)(&h_j);
                }

                HashChain::new(sp, x_j)
            })
            .collect();

        table.sort_unstable();

        RainbowTable {
            table,
            t,
            m,
            hash_function,
            reductions,
        }
    }

    fn compute_chain(&self, chain: &HashChain, r: u32) -> Text {
        assert!(r < self.t);

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
