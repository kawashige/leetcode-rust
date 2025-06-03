use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut status = status;
        let mut queue = VecDeque::new();
        let mut have_boxes = vec![false; status.len()];
        for b in initial_boxes {
            have_boxes[b as usize] = true;
            if status[b as usize] == 1 {
                queue.push_back(b as usize);
            }
        }

        let mut result = 0;
        let mut seen = vec![false; status.len()];
        while let Some(b) = queue.pop_front() {
            if status[b] == 0 || seen[b] {
                continue;
            }
            seen[b] = true;
            result += candies[b];

            for k in &keys[b] {
                status[*k as usize] = 1;
                if !seen[*k as usize] && have_boxes[*k as usize] {
                    queue.push_back(*k as usize);
                }
            }
            for b in &contained_boxes[b] {
                have_boxes[*b as usize] = true;
                if !seen[*b as usize] && status[*b as usize] == 1 {
                    queue.push_back(*b as usize);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1298() {
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 100],
                vec![vec![], vec![], vec![1], vec![]],
                vec![vec![1, 2], vec![3], vec![], vec![]],
                vec![0]
            ),
            16
        );
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1],
                vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
                vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
                vec![0]
            ),
            6
        );
    }
}
