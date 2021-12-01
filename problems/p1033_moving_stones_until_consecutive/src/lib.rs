pub struct Solution {}

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut stones = vec![a, b, c];
        stones.sort_unstable();

        vec![
            if stones[2] - stones[0] == 2 {
                0
            } else if stones[1] - stones[0] < 3 || stones[2] - stones[1] < 3 {
                1
            } else {
                2
            },
            if stones[2] - stones[0] == 2 {
                0
            } else {
                stones[2] - stones[0] - 2
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1033() {
        assert_eq!(Solution::num_moves_stones(1, 2, 5), vec![1, 2]);
        assert_eq!(Solution::num_moves_stones(4, 3, 2), vec![0, 0]);
        assert_eq!(Solution::num_moves_stones(3, 5, 1), vec![1, 2]);
    }
}
