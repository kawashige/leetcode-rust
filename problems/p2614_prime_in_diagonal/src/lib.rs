pub struct Solution {}

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut diagonal = Vec::with_capacity(nums.len() * 2);
        for i in 0..nums.len() {
            diagonal.push(nums[i][i]);
            diagonal.push(nums[nums.len() - 1 - i][i]);
        }
        diagonal.sort_unstable();

        let mut is_prime = vec![true; *diagonal.last().unwrap() as usize + 1];
        is_prime[1] = false;
        for i in 2..is_prime.len() {
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }
        diagonal
            .into_iter()
            .rev()
            .find(|num| is_prime[*num as usize])
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2614() {
        assert_eq!(
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]]),
            11
        );
        assert_eq!(
            Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 11, 10]]),
            17
        );
    }
}
