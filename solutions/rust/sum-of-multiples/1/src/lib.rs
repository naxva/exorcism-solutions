use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();
    for factor in factors {
        let cur_multiples: HashSet<u32> = (1..limit)
            .filter(|i| i.is_multiple_of(*factor))
            .collect();
        multiples.extend(&cur_multiples);
    }
    multiples.iter().sum()
}
