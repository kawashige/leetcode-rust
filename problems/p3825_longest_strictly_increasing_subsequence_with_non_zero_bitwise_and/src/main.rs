pub struct Solution {}

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for b in 0..=30 {
            let mut tails = Vec::new();

            for i in 0..nums.len() {
                if nums[i] & 1 << b == 0 {
                    continue;
                }
                match tails.binary_search(&nums[i]) {
                    Ok(_) => continue,
                    Err(j) if tails.len() == j => tails.push(nums[i]),
                    Err(j) => tails[j] = nums[i],
                }
                result = result.max(tails.len() as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3825() {
        assert_eq!(Solution::longest_subsequence(vec![5, 4, 7]), 2);
        assert_eq!(Solution::longest_subsequence(vec![2, 3, 6]), 3);
        assert_eq!(Solution::longest_subsequence(vec![0, 1]), 1);
    }
}

fn main() {}
