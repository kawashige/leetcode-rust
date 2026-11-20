pub struct Solution {}

impl Solution {
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut count = vec![0; 1_000_001];
        let mut result = 1;

        for n in nums {
            let tmp = count[n as usize];
            count[n as usize] = count[n as usize].max(count[n as usize - 1] + 1);
            count[n as usize + 1] = count[n as usize + 1].max(tmp + 1);
            result = result.max(count[n as usize]);
            result = result.max(count[n as usize + 1]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3041() {
        assert_eq!(Solution::max_selected_elements(vec![2, 1, 5, 1, 1]), 3);
        assert_eq!(Solution::max_selected_elements(vec![1, 4, 7, 10]), 1);
    }
}

fn main() {}
