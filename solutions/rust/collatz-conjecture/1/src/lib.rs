pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n => {
            if n.is_multiple_of(2) {
                Some(1 + collatz(n / 2).unwrap())
            } else {
                Some(1 + collatz(3 * n + 1).unwrap())
            }
        }
    }
}
