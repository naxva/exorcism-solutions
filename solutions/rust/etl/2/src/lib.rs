use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output = BTreeMap::new();
    for (points, letters) in input {
        for letter in letters {
            output.insert(letter.to_ascii_lowercase(), *points);
        }
    }

    output
}
