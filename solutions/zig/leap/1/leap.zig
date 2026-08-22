/// A year is a leap year if:
///     - It is divisible by 4
///     - It is NOT divisible by 100
///         - EXCEPT if it is divisible by 400
pub fn isLeapYear(year: u32) bool {
    if (year % 4 == 0) {
        if (year % 100 == 0) {
            return (year % 400 == 0);
        }
        return true;
    }
    return false;
}
