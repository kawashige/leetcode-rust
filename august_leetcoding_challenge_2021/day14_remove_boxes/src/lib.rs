pub struct Solution {}

impl Solution {
    pub fn recurse(
        s: usize,
        e: usize,
        k: usize,
        boxes: &Vec<i32>,
        memo: &mut Vec<Vec<Vec<usize>>>,
    ) -> usize {
        if s > e {
            return 0;
        }

        if memo[s][e][k] > 0 {
            return memo[s][e][k];
        }

        memo[s][e][k] = if e > 0 {
            Self::recurse(s, e - 1, 0, boxes, memo)
        } else {
            0
        } + (k + 1) * (k + 1);
        for i in s..e {
            if boxes[i] == boxes[e] {
                memo[s][e][k] = memo[s][e][k].max(
                    Self::recurse(s, i, k + 1, boxes, memo)
                        + Self::recurse(i + 1, e - 1, 0, boxes, memo),
                );
            }
        }

        memo[s][e][k]
    }

    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut memo = vec![vec![vec![0; 100]; 100]; 100];
        Self::recurse(0, boxes.len() - 1, 0, &boxes, &mut memo) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day14() {
        assert_eq!(
            Solution::remove_boxes(vec![
                1, 2, 2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 1,
                2, 1, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 1, 2, 2,
                1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1,
                1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1
            ]),
            2758
        );
        assert_eq!(Solution::remove_boxes(vec![1, 2]), 2);
        assert_eq!(Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
        assert_eq!(Solution::remove_boxes(vec![1, 1, 1]), 9);
        assert_eq!(Solution::remove_boxes(vec![1]), 1);
    }
}
