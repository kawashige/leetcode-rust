pub struct Solution {}

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as usize;
        let second_len = second_len as usize;

        let mut l_first = vec![0; nums.len()];
        let mut sum = (0..first_len).map(|i| nums[i]).sum::<i32>();
        l_first[first_len - 1] = sum;
        for i in (first_len)..nums.len() {
            sum += nums[i] - nums[i - first_len];
            l_first[i] = sum.max(l_first[i - 1]);
        }

        let mut r_first = vec![0; nums.len() + 1];
        let mut sum = ((nums.len() - first_len)..nums.len())
            .map(|i| nums[i])
            .sum::<i32>();
        r_first[nums.len() - first_len] = sum;
        for i in (0..(nums.len() - first_len)).rev() {
            sum += nums[i] - nums[i + first_len];
            r_first[i] = sum.max(r_first[i + 1]);
        }

        let mut sum = (0..second_len).map(|i| nums[i]).sum::<i32>();
        let mut r = sum + r_first[second_len];

        for i in (second_len)..nums.len() {
            sum += nums[i] - nums[i - second_len];
            r = r.max(sum + l_first[i - second_len].max(r_first[i + 1]));
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1031() {
        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
            20
        );
        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
            29
        );
        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
            31
        );
    }
}
