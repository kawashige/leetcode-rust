pub struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut error_nums = Vec::with_capacity(2);
        let mut seen = vec![false; nums.len() + 1];
        for n in nums {
            if seen[n as usize] {
                error_nums.push(n);
            } else {
                seen[n as usize] = true;
            }
        }
        error_nums.push((1..seen.len()).find(|j| !seen[*j]).unwrap() as i32);
        if seen[0] > seen[1] {
            seen.swap(0, 1)
        }
        error_nums
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0645() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
