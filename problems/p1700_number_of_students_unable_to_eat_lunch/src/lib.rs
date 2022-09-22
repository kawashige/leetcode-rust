use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = VecDeque::from(students);

        for i in 0..sandwiches.len() {
            let l = students.len();
            let mut count = 0;
            while count < l && sandwiches[i] != students[0] {
                let v = students.pop_front().unwrap();
                students.push_back(v);
                count += 1;
            }

            if sandwiches[i] != students[0] {
                return students.len() as i32;
            }
            students.pop_front();
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1700() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
