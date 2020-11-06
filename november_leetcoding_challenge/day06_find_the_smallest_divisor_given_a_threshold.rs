pub struct Solution {}

impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        fn check(nums: &Vec<i32>, threshold: i32, divisor: i32) -> bool {
            nums.iter()
                .fold(0, |acc, i| acc + (*i as f32 / divisor as f32).ceil() as i32)
                <= threshold
        }

        let mut i = 2;
        let mut j = *nums.iter().max().unwrap();
        while i <= j {
            let mid = i + (j - i) / 2;
            let prev = check(&nums, threshold, mid - 1);
            let cur = check(&nums, threshold, mid);

            if !prev && cur {
                return mid;
            } else if prev && cur {
                j = mid - 1;
            } else {
                i = mid + 1;
            }
        }
        1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day6() {
        assert_eq!(
            495280,
            Solution::smallest_divisor(vec![962551, 933661, 905225, 923035, 990560], 10)
        );
        assert_eq!(1, Solution::smallest_divisor(vec![1, 2, 3], 6));
        assert_eq!(5, Solution::smallest_divisor(vec![1, 2, 5, 9], 6));
        assert_eq!(3, Solution::smallest_divisor(vec![2, 3, 5, 7, 11], 11));
        assert_eq!(4, Solution::smallest_divisor(vec![19], 5));
    }
}
