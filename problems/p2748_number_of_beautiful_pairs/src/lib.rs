pub struct Solution {}

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut first = nums[i];
            while 10 <= first {
                first /= 10;
            }
            for j in i + 1..nums.len() {
                let last = nums[j] % 10;
                if (first != 1 && first == last)
                    || (first % 2 == 0 && last % 2 == 0)
                    || (first % 3 == 0 && last % 3 == 0)
                {
                    continue;
                }
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2748() {
        assert_eq!(Solution::count_beautiful_pairs(vec![2, 5, 1, 4]), 5);
        assert_eq!(Solution::count_beautiful_pairs(vec![11, 21, 12]), 2);
    }
}
