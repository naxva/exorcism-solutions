pub const NucleotideError = error{Invalid};

pub const Counts = struct {
    a: u32,
    c: u32,
    g: u32,
    t: u32,
};

pub fn countNucleotides(s: []const u8) NucleotideError!Counts {
    var counts = Counts{ .a = 0, .c = 0, .g = 0, .t = 0 };
    for (s) |char| {
        if (char - 'A' == 0) {
            counts.a += 1;
            continue;
        }
        if (char - 'C' == 0) {
            counts.c += 1;
            continue;
        }
        if (char - 'G' == 0) {
            counts.g += 1;
            continue;
        }
        if (char - 'T' == 0) {
            counts.t += 1;
            continue;
        } else {
            return NucleotideError.Invalid;
        }
    }

    return counts;
}
