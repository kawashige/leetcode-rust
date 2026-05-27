pub struct Solution {}

impl Solution {
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = [0; 101];
        for n in nums {
            count[n as usize] += 1;
        }
        (1..101).fold(0, |acc, i| {
            acc + i as i32 * if count[i] % k == 0 { count[i] } else { 0 }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3712() {
        assert_eq!(
            Solution::sum_divisible_by_k(vec![1, 2, 2, 3, 3, 3, 3, 4], 2),
            16
        );
        assert_eq!(Solution::sum_divisible_by_k(vec![1, 2, 3, 4, 5], 2), 0);
        assert_eq!(Solution::sum_divisible_by_k(vec![4, 4, 4, 1, 2, 3], 3), 12);
    }
}

fn main() {}
