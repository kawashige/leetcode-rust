pub struct Solution {}

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let count = nums.into_iter().fold([0; 51], |mut count, n| {
            count[n as usize] += 1;
            count
        });
        (0..count.len()).fold(0, |acc, i| if count[i] == 2 { acc ^ i as i32 } else { acc })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3158() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
    }
}
