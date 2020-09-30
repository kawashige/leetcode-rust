pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut nums = nums;
        for i in 0..nums.len() {
            if nums[i] < 1 || n < nums[i] {
                nums[i] = 0;
            } else if nums[i] != i as i32 + 1 {
                let mut j = nums[i] as usize - 1;
                nums[i] = 0;
                while j as i32 + 1 != nums[j] {
                    let tmp = nums[j];
                    nums[j] = j as i32 + 1;
                    if tmp < 1 || n < tmp {
                        break;
                    }
                    j = tmp as usize - 1;
                }
            }
        }

        nums.into_iter()
            .enumerate()
            .find(|(_, n)| *n == 0)
            .unwrap_or((n as usize, 1))
            .0 as i32
            + 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day30() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
        assert_eq!(4, Solution::first_missing_positive(vec![1, 2, 3]));
        assert_eq!(1, Solution::first_missing_positive(vec![2, 2, 2]));
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    }
}
