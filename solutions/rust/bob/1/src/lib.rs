pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    let is_yelling = message
        .chars()
        .filter(char::is_ascii_alphabetic)
        .all(char::is_uppercase)
        && message.chars().filter(char::is_ascii_alphabetic).count() > 0;
    let is_question = message.trim().ends_with('?');

    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (false, false) => "Whatever.",
    }
}
