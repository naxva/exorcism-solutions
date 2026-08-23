// Your task is to figure out if a sentence is a pangram.
//
// A pangram is a sentence using every letter of the alphabet at least once.
// It is case insensitive, so it doesn't matter if a letter is lower-case (e.g. `k`) or upper-case (e.g. `K`).
//
// For this exercise, a sentence is a pangram if it contains each of the 26 letters in the English alphabet.

const std = @import("std");

pub fn isPangram(str: []const u8) bool {
    var letters: [26]bool = .{false} ** 26;
    for (str) |char| {
        if (std.ascii.isAlphabetic(char)) {
            const index = std.ascii.toLower(char) - 'a';
            letters[index] = true;
        }
    }

    for (letters) |state| {
        if (!state) return false;
    }

    return true;
}
