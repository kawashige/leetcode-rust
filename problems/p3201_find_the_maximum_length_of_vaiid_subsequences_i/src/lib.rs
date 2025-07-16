pub struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut count = [0; 2];
        let mut max_len = [0; 2];

        for i in 0..nums.len() {
            let p1 = (nums[i] % 2) as usize;
            let p2 = ((nums[i] + 1) % 2) as usize;
            count[p1] += 1;
            max_len[p2] = max_len[p2].max(max_len[p1] + 1);
        }

        count[0].max(count[1]).max(max_len[0]).max(max_len[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3201() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4]), 4);
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 2, 1, 2]), 6);
        assert_eq!(Solution::maximum_length(vec![1, 3]), 2);
    }
}
