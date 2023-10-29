pub struct Solution {}

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut count = 0;

        for i in 1..nums.len() - 1 {
            if nums[i] == nums[i - 1] {
                continue;
            }
            match (
                nums[..i].iter().rev().find(|num| num != &&nums[i]),
                nums[i + 1..].iter().find(|num| num != &&nums[i]),
            ) {
                (Some(num1), Some(num2)) => {
                    if (&nums[i] < num1 && &nums[i] < num2) || (num1 < &nums[i] && num2 < &nums[i])
                    {
                        count += 1;
                    }
                }
                _ => {}
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2210() {
        assert_eq!(Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
        assert_eq!(Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
    }
}
