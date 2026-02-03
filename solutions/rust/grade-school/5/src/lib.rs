use std::collections::{BTreeMap, HashMap, BTreeSet};
#[derive(Default)]
pub struct School {
    grade_students: BTreeMap<u32, BTreeSet<String>>,
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
        students.insert(student.to_string());
    }
    pub fn grades(&self) -> Vec<u32> {
        self.grade_students.keys().copied().collect()
    }
    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
       self.grade_students
        .get(&grade)
        .cloned()           // 這裡得到 Option<BTreeSet<String>>
        .unwrap_or_default() // 如果是 None 則返回空的 BTreeSet<String>
        .into_iter()         // 轉換成迭代器
        .collect()           // 收集成 Vec<String>
    }
}