use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    grade_students: BTreeMap<u32, Vec<String>>,
    student_grade: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.student_grade.contains_key(student) {
            return;
        }
        self.student_grade.insert(student.to_string(), grade);
        let students = self.grade_students.entry(grade).or_default();
        students.push(student.to_string());
        students.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grade_students.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grade_students.get(&grade).unwrap_or(&Vec::new()).to_vec()
    }
}
