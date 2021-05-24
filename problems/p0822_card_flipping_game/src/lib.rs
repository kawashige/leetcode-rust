pub struct Solution {}

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut both = vec![0; 2001];
        for i in 0..fronts.len() {
            if fronts[i] == backs[i] {
                both[fronts[i] as usize] = 2;
            } else {
                if both[fronts[i] as usize] == 0 {
                    both[fronts[i] as usize] = 1;
                }
                if both[backs[i] as usize] == 0 {
                    both[backs[i] as usize] = 1;
                }
            }
        }
        (1..both.len()).find(|i| both[*i] == 1).unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0822() {
        assert_eq!(
            Solution::flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3]),
            2
        );

        assert_eq!(
            Solution::flipgame(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            0
        );

        assert_eq!(Solution::flipgame(vec![10], vec![9]), 9);
    }
}
