use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let aliquot = (1..num).filter(|i| num.is_multiple_of(*i)).sum();
    match num.cmp(&aliquot) {
        Ordering::Less => Some(Classification::Abundant),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Deficient),
    }
}
