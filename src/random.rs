
use rand::{Rng, thread_rng};

pub fn random_range(min: usize, max: usize) -> usize {

    let mut range = thread_rng();

    range.gen_range(min..max)
}
