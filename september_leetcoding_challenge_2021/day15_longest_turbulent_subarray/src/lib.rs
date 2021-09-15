pub struct Solution {}

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut ans = 1;

        let mut c1 = 1;
        let mut c2 = 1;
        for i in 1..arr.len() {
            if (i % 2 == 0 && arr[i - 1] < arr[i]) || (i % 2 == 1 && arr[i - 1] > arr[i]) {
                c1 += 1;
            } else {
                ans = ans.max(c1);
                c1 = 1;
            }
            if (i % 2 == 1 && arr[i - 1] < arr[i]) || (i % 2 == 0 && arr[i - 1] > arr[i]) {
                c2 += 1;
            } else {
                ans = ans.max(c2);
                c2 = 1;
            }
        }
        ans.max(c1).max(c2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day15() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
        assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
        assert_eq!(
            Solution::max_turbulence_size(vec![0, 8, 45, 88, 48, 68, 28, 55, 17, 24]),
            8
        );
    }
}
