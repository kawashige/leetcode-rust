pub struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const M: i32 = 1_000_000_007;

        let mut sum = 0;
        let mut count = [0; 2];
        count[0] = 1;
        let mut result = 0;

        for num in arr {
            sum += num;
            result = (result + count[(1 - sum % 2) as usize]) % M;
            count[(sum % 2) as usize] += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1524() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
        assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    }
}
