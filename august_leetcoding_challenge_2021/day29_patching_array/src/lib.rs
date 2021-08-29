pub struct Solution {}

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut current: i64 = 0;
        let mut count = 0;
        for x in nums {
            while current < n as i64 && x as i64 - 1 > current {
                current += current + 1;
                count += 1;
            }
            if n as i64 <= current {
                break;
            }
            current += x as i64;
        }

        while current < n as i64 {
            current += current + 1;
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day29() {
        assert_eq!(
            Solution::min_patches(
                vec![1, 7, 21, 31, 34, 37, 40, 43, 49, 87, 90, 92, 93, 98, 99],
                12
            ),
            2
        );
        assert_eq!(Solution::min_patches(vec![1, 2, 32, 33], 2147483647), 28);
        assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
        assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
        assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
    }
}
