pub struct Solution {}

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut sum = 0;
        let mut result = std::i32::MAX;
        let k = k as i64;

        for i in 0..nums.len() {
            sum += nums[i] as i64;
            match stack.binary_search_by_key(&(sum - k), |&(a, _)| a) {
                Ok(j) => {
                    result = result.min((i - stack[j].1) as i32);
                }
                Err(j) => {
                    if j != 0 {
                        result = result.min((i - stack[j - 1].1) as i32);
                    }
                }
            }
            if k <= sum {
                result = result.min(i as i32 + 1);
            }

            while !stack.is_empty() && sum <= stack.last().unwrap().0 {
                stack.pop();
            }
            stack.push((sum, i));
        }

        if result == std::i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0862() {
        assert_eq!(
            Solution::shortest_subarray(vec![17, 85, 93, -45, -21], 150),
            2
        );
        assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
        assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1);
        assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3);
    }
}
