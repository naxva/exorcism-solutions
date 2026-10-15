use std::collections::BTreeMap;

type Grade = BTreeMap<u32, Vec<String>>;

#[derive(Default)]
pub struct School {
    grades: Grade,
}

impl School {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self
            .grades
            .values()
            .any(|s| s.contains(&student.to_string()))
        {
            return;
        }
        self.grades
            .entry(grade)
            .and_modify(|e| {
                e.push(student.to_string());
                e.sort_unstable();
            })
            .or_insert_with(|| vec![student.to_string()]);
    }

    #[must_use]
    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    #[must_use]
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades.get(&grade).unwrap_or(&Vec::new()).clone()
    }
}
