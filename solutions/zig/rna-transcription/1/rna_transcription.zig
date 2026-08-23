// Your task is to determine the RNA complement of a given DNA sequence.
//
// Both DNA and RNA strands are a sequence of nucleotides.
//
// The four nucleotides found in DNA are adenine (**A**), cytosine (**C**), guanine (**G**), and thymine (**T**).
//
// The four nucleotides found in RNA are adenine (**A**), cytosine (**C**), guanine (**G**), and uracil (**U**).
//
// Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:
//
// - `G` -> `C`
// - `C` -> `G`
// - `T` -> `A`
// - `A` -> `U`

const std = @import("std");
const mem = std.mem;

pub fn toRna(allocator: mem.Allocator, dna: []const u8) mem.Allocator.Error![]const u8 {
    const rna = try allocator.alloc(u8, dna.len);
    for (dna, rna) |dna_nucl, *rna_nucl| {
        rna_nucl.* = switch (dna_nucl) {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            else => unreachable,
        };
    }

    return rna;
}
