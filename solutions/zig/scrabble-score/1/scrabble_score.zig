// | Letter                       | Value |
// | ---------------------------- | ----- |
// | A, E, I, O, U, L, N, R, S, T | 1     |
// | D, G                         | 2     |
// | B, C, M, P                   | 3     |
// | F, H, V, W, Y                | 4     |
// | K                            | 5     |
// | J, X                         | 8     |
// | Q, Z                         | 10    |

const std = @import("std");

pub fn score(s: []const u8) u32 {
    var points: u32 = 0;
    const lettersV = [_][]const u8{
        "aeioulnrst",
        "dg",
        "bcmp",
        "fhvwy",
        "k",
        "jx",
        "qz",
    };
    const scores = [_]u32{ 1, 2, 3, 4, 5, 8, 10 };

    for (s) |char| {
        const lowerchar = std.ascii.toLower(char);
        for (lettersV, scores) |letters, letterValue| {
            if (std.mem.indexOfScalar(u8, letters, lowerchar) != null) {
                points += letterValue;
                break;
            }
        }
    }

    return points;
}
