pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        fn find(nums: &[i32], minus: &[usize]) -> i32 {
            if nums.len() == 1 {
                nums[0]
            } else if minus.len() % 2 == 0 {
                nums.iter().product()
            } else {
                std::cmp::max(
                    nums[..minus[minus.len() - 1]].iter().product(),
                    nums[(minus[0] + 1)..].iter().product(),
                )
            }
        }

        let mut results = Vec::new();

        let mut tmp = Vec::new();
        let mut minus = Vec::new();
        for n in nums {
            if n == 0 {
                if tmp.len() > 0 {
                    results.push(find(&tmp, &minus));
                }
                results.push(0);
                tmp.clear();
                minus.clear();
            } else {
                tmp.push(n);
                if n < 0 {
                    minus.push(tmp.len() - 1);
                }
            }
        }
        if tmp.len() > 0 {
            results.push(find(&tmp, &minus));
        }
        results.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day11() {
        assert_eq!(6, Solution::max_product(vec![2, 3, -2, 4]));
        assert_eq!(0, Solution::max_product(vec![-2, 0, -1]));
    }
}
