pub struct Solution {}

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut sums = vec![0; k as usize];
        let mut sum = 0;
        let mut r = 0;

        for num in nums {
            sum += num;
            let i = (if sum >= 0 {
                sum % k
            } else {
                sum + ((sum.abs() + k - 1) / k) * k
            }) as usize;

            r += sums[i];
            if i == 0 {
                r += 1;
            }
            sums[i] += 1;
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0974() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
        assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
    }
}
