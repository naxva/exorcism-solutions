use std::collections::HashSet;

#[must_use]
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let sorted_word = sort_word(&word);
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate = candidate.to_lowercase();
            candidate.len() == word.len()
                && candidate != word
                && sort_word(&candidate) == sorted_word
        })
        .copied()
        .collect()
}

fn sort_word(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
