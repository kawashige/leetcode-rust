pub struct Solution {}

impl Solution {
    pub fn max_jump(stones: Vec<i32>) -> i32 {
        if stones.len() <= 3 {
            return stones.last().unwrap() - stones[0];
        }

        let odd = (1..stones.len() - 1)
            .step_by(2)
            .map(|i| stones[(i + 2).min(stones.len() - 1)] - stones[i])
            .max()
            .unwrap()
            .max(stones[1] - stones[0]);
        let even = (2..stones.len() - 1)
            .step_by(2)
            .map(|i| stones[(i + 2).min(stones.len() - 1)] - stones[i])
            .max()
            .unwrap()
            .max(stones[2] - stones[0]);

        odd.max(even)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2498() {
        assert_eq!(Solution::max_jump(vec![0, 2, 5, 6, 7]), 5);
        assert_eq!(Solution::max_jump(vec![0, 3, 9]), 9);
    }
}
