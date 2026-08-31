use primes::{PrimeSet, Sieve};
pub fn nth(n: u32) -> u32 {
    let mut pset = Sieve::new();
    pset.iter().nth(n as usize).unwrap() as u32
}
