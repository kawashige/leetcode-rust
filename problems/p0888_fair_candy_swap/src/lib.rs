pub struct Solution {}

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let mut sum = alice_sizes.iter().sum::<i32>();
        let mut nums = vec![false; 100_001];
        for i in 0..bob_sizes.len() {
            sum -= bob_sizes[i];
            nums[bob_sizes[i] as usize] = true;
        }

        for i in 0..alice_sizes.len() {
            if 0 < alice_sizes[i] - sum / 2
                && alice_sizes[i] - sum / 2 < nums.len() as i32
                && nums[(alice_sizes[i] - sum / 2) as usize]
            {
                return vec![alice_sizes[i], alice_sizes[i] - sum / 2];
            }
        }

        Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0888() {
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 1], vec![2, 2]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 2], vec![2, 3]),
            vec![1, 2]
        );
        assert_eq!(Solution::fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 2, 5], vec![2, 4]),
            vec![5, 4]
        );
    }
}
