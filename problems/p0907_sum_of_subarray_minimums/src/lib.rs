pub struct Solution {}

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const M: usize = 1_000_000_007;

        let mut r = 0;

        for i in 0..arr.len() {
            let mut s = i;
            while 0 < s && arr[s - 1] > arr[i] {
                s -= 1;
            }
            let mut e = i;
            while e < arr.len() - 1 && arr[e + 1] >= arr[i] {
                e += 1;
            }

            r += ((i - s + 1) * (e - i + 1) % M) * arr[i] as usize % M;
            r %= M;
        }

        r as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0907() {
        assert_eq!(Solution::sum_subarray_mins(vec![1, 1]), 3);
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
}
