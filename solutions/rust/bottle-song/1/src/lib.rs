struct NumbertoWords;

impl NumbertoWords {
    fn convert(number: u32) -> &'static str {
        match number {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            0 => "",
            _ => unreachable!(),
        }
    }
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn verse_builder(bottles: u32) -> String {
    let bottle_word = NumbertoWords::convert(bottles);
    let next_bottle_word = NumbertoWords::convert(bottles - 1);
    let capital_word = capitalize_first(bottle_word);
    let plural = if bottles == 1 { "bottle" } else { "bottles" };

    let more_bottles = match bottles {
        2 => " green bottle",
        1 => "no green bottles",
        _ => " green bottles",
    };

    format!(
        "{capital_word} green {plural} hanging on the wall,\n\
        {capital_word} green {plural} hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {next_bottle_word}{more_bottles} hanging on the wall.",
    )
}
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();
    for bottle in ((start_bottles - take_down) + 1..=start_bottles).rev() {
        verses.push(verse_builder(bottle));
    }

    verses.join("\n\n")
}
