pub struct Solution {}

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut nums = Vec::new();
        let mut cost: i64 = 0;

        for n in instructions {
            match nums.binary_search(&n) {
                Ok(i) => {
                    let j = (0..i as i32)
                        .rev()
                        .find(|j| nums[*j as usize] != n)
                        .unwrap_or(-1);
                    let k = (i..nums.len())
                        .find(|j| nums[*j as usize] != n)
                        .unwrap_or(nums.len());
                    cost += std::cmp::min(j + 1, (nums.len() - k) as i32) as i64;
                    nums.insert(i, n);
                }
                Err(i) => {
                    cost += std::cmp::min(i, nums.len() - i) as i64;
                    nums.insert(i, n);
                }
            }
        }

        (cost % (1_000_000_000 + 7)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day10() {
        assert_eq!(0, Solution::create_sorted_array(vec![1]));
        assert_eq!(1, Solution::create_sorted_array(vec![1, 5, 6, 2]));
        assert_eq!(3, Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]));
        assert_eq!(
            4,
            Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2])
        );
    }
}
