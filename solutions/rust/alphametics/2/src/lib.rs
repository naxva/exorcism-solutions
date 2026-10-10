use std::collections::{HashMap, HashSet};

fn word_value(word: &str, map: &HashMap<char, u8>) -> u64 {
    let mut sum: u64 = 0;
    for ch in word.chars() {
        sum = sum * 10 + u64::from(map[&ch]);
    }
    sum
}

struct Puzzle<'a> {
    addends: Vec<&'a str>,
    result: &'a str,
    letters: Vec<char>,
    leading: HashSet<char>,
}

fn is_solution(p: &Puzzle, map: &HashMap<char, u8>) -> bool {
    let left: u64 = p.addends.iter().map(|w| word_value(w, map)).sum();
    left == word_value(p.result, map)
}

fn search(p: &Puzzle, idx: usize, used: &mut [bool; 10], map: &mut HashMap<char, u8>) -> bool {
    // every letter has a digit: just test it
    if idx == p.letters.len() {
        return is_solution(p, map);
    }

    let letter = p.letters[idx];
    let cannot_be_zero = p.leading.contains(&letter);

    for digit in 0..=9u8 {
        let taken = used[digit as usize];
        if taken || (digit == 0 && cannot_be_zero) {
            continue;
        }
        used[digit as usize] = true; // choose
        map.insert(letter, digit);
        if search(p, idx + 1, used, map) {
            return true;
        }
        used[digit as usize] = false; //undo
    }
    false
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==")?;
    let addends: Vec<&str> = left.split('+').map(str::trim).collect();
    let result = right.trim();

    let letters: Vec<char> = addends
        .iter()
        .copied()
        .chain([result])
        .flat_map(str::chars)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    if letters.len() > 10 {
        return None;
    }
    let leading = addends
        .iter()
        .copied()
        .chain([result])
        .filter_map(|w| w.chars().next())
        .collect();

    let p = Puzzle {
        addends,
        result,
        letters,
        leading,
    };
    let mut map = HashMap::new();
    search(&p, 0, &mut [false; 10], &mut map).then_some(map)
}
