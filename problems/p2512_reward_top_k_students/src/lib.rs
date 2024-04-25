use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let mut point = HashMap::new();
        let positive = positive_feedback.into_iter().collect::<HashSet<_>>();
        let negative = negative_feedback.into_iter().collect::<HashSet<_>>();

        for i in 0..report.len() {
            let p = report[i]
                .split_ascii_whitespace()
                .map(|w| {
                    if positive.contains(w) {
                        3
                    } else if negative.contains(w) {
                        -1
                    } else {
                        0
                    }
                })
                .sum::<i32>();
            *point.entry(student_id[i]).or_insert(0) += p
        }

        let mut point = point.into_iter().collect::<Vec<_>>();
        point.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        point
            .into_iter()
            .take(k as usize)
            .map(|(id, _)| id)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2512() {
        assert_eq!(
            Solution::top_students(
                vec![
                    "smart".to_string(),
                    "brilliant".to_string(),
                    "studious".to_string()
                ],
                vec!["not".to_string()],
                vec![
                    "this student is studious".to_string(),
                    "the student is smart".to_string()
                ],
                vec![1, 2],
                2
            ),
            vec![1, 2]
        );
        assert_eq!(
            Solution::top_students(
                vec![
                    "smart".to_string(),
                    "brilliant".to_string(),
                    "studious".to_string()
                ],
                vec!["not".to_string()],
                vec![
                    "this student is not studious".to_string(),
                    "the student is smart".to_string()
                ],
                vec![1, 2],
                2
            ),
            vec![2, 1]
        );
    }
}
