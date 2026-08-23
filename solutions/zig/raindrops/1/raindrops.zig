// Your task is to convert a number into its corresponding raindrop sounds.
//
// If a given number:
//
// - is divisible by 3, add "Pling" to the result.
// - is divisible by 5, add "Plang" to the result.
// - is divisible by 7, add "Plong" to the result.
// - **is not** divisible by 3, 5, or 7, the result should be the number as a string.

// This is a variation of FizzBuzz, where we don't count up or down in a range

const std = @import("std");

pub fn convert(buffer: []u8, n: u32) []const u8 {
    var writer: std.Io.Writer = .fixed(buffer);
    if (n % 3 == 0) writer.writeAll("Pling") catch unreachable;
    if (n % 5 == 0) writer.writeAll("Plang") catch unreachable;
    if (n % 7 == 0) writer.writeAll("Plong") catch unreachable;
    if (n % 3 != 0 and n % 5 != 0 and n % 7 != 0) writer.print("{d}", .{n}) catch unreachable;

    return buffer[0..writer.end];
}
