/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut total = 0;
    // the number of characters in a valid ISBN string
    let mut num_chars = 10;
    for ch in isbn.chars() {
        match ch {
            '-' => continue,
            'X' if num_chars == 1 => total += 10,
            '0'..='9' => total += ch.to_digit(10).unwrap() * num_chars,
            _ => return false,
        }

        // There are more than 10 characters
        if num_chars == 0 {
            return false;
        }
        num_chars -= 1;
    }

    total % 11 == 0 && num_chars == 0
}
