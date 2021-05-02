use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut heap = BinaryHeap::new();
        let mut t = 0;
        for course in courses {
            t += course[0];
            heap.push(course[0]);
            while t > course[1] {
                if let Some(d) = heap.pop() {
                    t -= d;
                }
            }
        }
        heap.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day02() {
        assert_eq!(
            Solution::schedule_course(vec![vec![1, 19], vec![2, 2], vec![1, 17]]),
            3
        );
        assert_eq!(
            Solution::schedule_course(vec![
                vec![7, 16],
                vec![2, 3],
                vec![3, 12],
                vec![3, 14],
                vec![10, 19],
                vec![10, 16],
                vec![6, 8],
                vec![6, 11],
                vec![3, 13],
                vec![6, 16]
            ]),
            4
        );
        assert_eq!(
            Solution::schedule_course(vec![
                vec![100, 200],
                vec![200, 1300],
                vec![1000, 1250],
                vec![2000, 3200]
            ]),
            3
        );
        assert_eq!(Solution::schedule_course(vec![vec![1, 2]]), 1);
        assert_eq!(Solution::schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
    }
}
