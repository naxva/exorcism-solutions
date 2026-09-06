pub fn build_proverb(list: &[&str]) -> String {
    let mut res = String::new();
    if list.is_empty() {
        return res;
    }
    let iter = list.windows(2);
    for window in iter {
        let next_line = format!("For want of a {} the {} was lost.\n", window[0], window[1]);
        res.push_str(&next_line);
    }
    res.push_str(&format!("And all for the want of a {}.", list[0]));
    res
}
