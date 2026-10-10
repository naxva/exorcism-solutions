use std::cmp::Ordering;

#[must_use]
pub fn find<T: Ord, V: AsRef<[T]>>(array: V, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mid = array.len() / 2;
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(array.get(..mid)?, key),
        Ordering::Greater => find(array.get(mid.saturating_add(1)..)?, key)
            .map(|i| i.saturating_add(mid).saturating_add(1)),
    }
}
