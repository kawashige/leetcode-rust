use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let walls = wall.len();
        let mut lines = HashMap::new();
        for w in wall {
            let mut sum = 0;
            for i in 0..(w.len() - 1) {
                sum += w[i];
                *lines.entry(sum).or_insert(0) += 1;
            }
        }
        (walls - lines.values().max().unwrap_or(&0)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0554() {
        assert_eq!(
            Solution::least_bricks(vec![
                vec![1, 2, 2, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 4],
                vec![3, 1, 2],
                vec![1, 3, 1, 1]
            ]),
            2
        );
        assert_eq!(Solution::least_bricks(vec![vec![10]]), 1);
    }
}
