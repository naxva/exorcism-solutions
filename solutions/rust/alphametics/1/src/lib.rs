use std::collections::HashMap;

// Each letter in the puzzle is reduced to a single signed integer `weight`:
// its net place-value coefficient across all words, positive for LHS addends,
// negative for the RHS result. For example in SEND + MORE == MONEY:
//   S contributes +1000 (from SEND), so weight(S) = +1000
//   M contributes +1000 (from MORE) and -10000 (from MONEY), so weight(M) = -9000
//
// The puzzle is then just: sum of (weight[letter] * digit[letter]) == 0.
//
// min_suffix / max_suffix are precomputed bounds: the minimum and maximum value
// that all letters *after* this one in the search order could together contribute.
// Used to prune branches that can no longer reach zero.
struct Letter {
    letter: char,
    weight: i64,     // net place-value coefficient (LHS positive, RHS negative)
    min_suffix: i64, // minimum possible sum from this letter onward (all remaining × 9)
    max_suffix: i64, // maximum possible sum from this letter onward (all remaining × 9)
    non_zero: bool,  // true if this letter is a leading digit (cannot be assigned 0)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (lhs, rhs) = input.split_once("==")?;

    // Index 0..26 corresponds to letters A..Z.
    let mut weights = [0i64; 26];
    let mut leading = [false; 26]; // tracks which letters appear as the first char of a word
    let mut present = 0u32; // bitmask of which letters appear at all

    // Walk each word on both sides. LHS words add to the equation (sign +1),
    // the RHS word subtracts (sign -1). We compute each letter's net weight by
    // iterating the word in reverse so place values are 1, 10, 100, etc.
    for (side, sign) in [(lhs, 1i64), (rhs, -1i64)] {
        for raw_word in side.split('+') {
            let word = raw_word.trim();
            if word.is_empty() {
                continue;
            }

            let mut place = sign; // starts at ±1 (units), then ±10, ±100, ...
            for &b in word.as_bytes().iter().rev() {
                let idx = (b - b'A') as usize;
                weights[idx] += place;
                present |= 1 << idx; // mark this letter as seen
                place *= 10;
            }

            // The first character of every word cannot be zero (no leading zeroes).
            let first_idx = (word.as_bytes()[0] - b'A') as usize;
            leading[first_idx] = true;
        }
    }

    // A valid puzzle must have between 1 and 10 distinct letters
    // (there are only 10 digits, so more than 10 letters is unsolvable).
    if present == 0 || present.count_ones() > 10 {
        return None;
    }

    // Collect only the letters that actually appear in the puzzle.
    let mut letters = Vec::new();
    for i in 0..26 {
        if present & (1 << i) != 0 {
            letters.push(Letter {
                letter: (b'A' + i as u8) as char,
                weight: weights[i],
                min_suffix: 0, // filled in below
                max_suffix: 0, // filled in below
                non_zero: leading[i],
            });
        }
    }

    // Most-constrained variable first: sort by descending |weight| so that the
    // letters with the largest impact on the total sum are assigned early.
    // This causes failed branches to be discovered as high up the tree as possible,
    // pruning enormous subtrees before they are explored.
    letters.sort_unstable_by_key(|l| -l.weight.abs());

    // Precompute suffix reachability bounds, scanning from the last letter backward.
    // For each letter at position i, min_suffix / max_suffix represent the range of
    // sums that letters [i..end] could contribute if each were assigned any digit 0–9.
    // These let backtrack() prune branches where the total can never reach zero,
    // even if all remaining letters cooperate as much as possible.
    let mut min_sum = 0;
    let mut max_sum = 0;
    for letter in letters.iter_mut().rev() {
        if letter.weight > 0 {
            max_sum += letter.weight * 9; // largest possible positive contribution
        } else {
            min_sum += letter.weight * 9; // largest possible negative contribution
        }
        letter.min_suffix = min_sum;
        letter.max_suffix = max_sum;
    }

    // Run the backtracking search. On success, `digits[i]` holds the assigned
    // digit for `letters[i]` in sort order.
    let mut digits = [0u8; 10];
    if !backtrack(&letters, 0, 0, &mut digits, 0) {
        return None;
    }

    // Build and return the result map from letter → digit.
    let mut solution = HashMap::new();
    for i in 0..letters.len() {
        solution.insert(letters[i].letter, digits[i]);
    }
    Some(solution)
}

// Recursive depth-first search assigning one digit per letter.
//
// Arguments:
//   letters  - the remaining letters to assign, in sort order
//   sum      - accumulated weighted sum of digits assigned so far
//   used     - bitmask of digits 0–9 already taken (bit i = digit i is used)
//   digits   - output array; digits[depth] is written on the way back up
//   depth    - index into `digits` for the current letter
//
// Returns true if a valid complete assignment was found (sum == 0 at the leaves).
fn backtrack(letters: &[Letter], sum: i64, used: u16, digits: &mut [u8; 10], depth: usize) -> bool {
    let Some((first, rest)) = letters.split_first() else {
        // Base case: all letters assigned. Valid only if the weighted sum is zero.
        return sum == 0;
    };

    // Look up the reachability bounds for letters *after* the current one.
    // If `rest` is empty (this is the last letter), both bounds are 0.
    let mut min_rest = 0;
    let mut max_rest = 0;
    if let Some(next) = rest.first() {
        min_rest = next.min_suffix;
        max_rest = next.max_suffix;
    }

    // Build the set of candidate digits for this letter.
    // !used flips all bits so set bits represent available digits 0–9.
    // 0x03FF masks to just bits 0–9 (the ten valid digit positions).
    // If this letter cannot be zero (leading digit), also clear bit 0.
    let mut available = !used & 0x03FF;
    if first.non_zero {
        available &= !1; // clear bit 0, forbidding digit 0
    }

    // Iterate over available digits from smallest to largest using bit tricks.
    // trailing_zeros() gives the position of the lowest set bit (cheapest digit to try).
    // `available &= available - 1` clears that lowest set bit (BLSR instruction),
    // advancing to the next candidate in one cycle.
    while available != 0 {
        let digit = available.trailing_zeros() as u8;
        available &= available - 1;

        let next_sum = sum + first.weight * digit as i64;

        // Monotonic cutoff: digits are iterated in increasing order, so next_sum
        // moves in a fixed direction as digit grows. Once it overshoots zero
        // beyond what the remaining letters can correct, all larger digits will
        // also overshoot — we can break out of the loop entirely.
        if next_sum + min_rest > 0 && first.weight > 0 {
            break; // increasing digit only makes a positive-weight sum more positive
        }
        if next_sum + max_rest < 0 && first.weight < 0 {
            break; // increasing digit only makes a negative-weight sum more negative
        }

        // Window reachability check: even with this digit, the remaining letters
        // cannot bring the total to zero — skip this digit but try the next.
        if next_sum + max_rest < 0 || next_sum + min_rest > 0 {
            continue;
        }

        // Recurse with this digit assigned. Pass `used | (1 << digit)` to mark
        // it taken. On success, record the digit on the way back up the stack.
        if backtrack(rest, next_sum, used | (1 << digit), digits, depth + 1) {
            digits[depth] = digit;
            return true;
        }
    }

    false
}
