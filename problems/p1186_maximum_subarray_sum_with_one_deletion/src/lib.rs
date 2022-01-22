pub struct Solution {}

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return arr[0];
        }

        let n = arr.len() - 1;
        let mut acc_left = vec![0; arr.len()];
        let mut acc_right = vec![0; arr.len()];
        acc_left[0] = arr[0];
        acc_right[n] = arr[n];
        for i in 1..arr.len() {
            acc_left[i] = arr[i] + acc_left[i - 1];
            acc_right[n - i] = arr[n - i] + acc_right[n - i + 1];
        }

        let mut left_max = vec![0; arr.len()];
        let mut right_max = vec![0; arr.len()];
        left_max[0] = acc_right[0];
        right_max[n] = acc_left[n];
        for i in 1..arr.len() {
            left_max[i] = left_max[i - 1].max(acc_right[i]);
            right_max[n - i] = right_max[n - i + 1].max(acc_left[n - i]);
        }

        let mut max = std::i32::MIN;
        for i in 0..arr.len() {
            let left = if i == 0 {
                0
            } else {
                left_max[i - 1] - acc_right[i]
            };
            let right = if i == n {
                0
            } else {
                right_max[i + 1] - acc_left[i]
            };

            max = max.max((left + right).max(left.max(0) + right.max(0) + arr[i]));
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1186() {
        assert_eq!(Solution::maximum_sum(vec![-50]), -50);
        assert_eq!(Solution::maximum_sum(vec![-50, 1]), 1);
        assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4);
        assert_eq!(Solution::maximum_sum(vec![1, -2, -2, 3]), 3);
        assert_eq!(Solution::maximum_sum(vec![-1, -1, -1, -1]), -1);
    }
}
