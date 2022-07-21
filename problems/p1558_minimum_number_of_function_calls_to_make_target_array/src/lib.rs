use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(num: i32, memo: &mut HashMap<i32, (i32, i32)>) -> (i32, i32) {
        if num == 0 {
            return (0, 0);
        }
        if let Some((mul, add)) = memo.get(&num) {
            return (*mul, *add);
        }

        if num % 2 == 0 {
            let (mul, add) = Self::recurse(num / 2, memo);
            memo.insert(num, (mul + 1, add));
            (mul + 1, add)
        } else {
            let (mul, add) = Self::recurse(num - 1, memo);
            memo.insert(num, (mul, add + 1));
            (mul, add + 1)
        }
    }

    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut max_mul_count = 0;
        let mut count = 0;
        let mut memo = HashMap::new();

        for num in nums {
            let (mul, add) = Self::recurse(num, &mut memo);
            count += add;
            max_mul_count = max_mul_count.max(mul);
        }

        count + max_mul_count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1558() {
        assert_eq!(Solution::min_operations(vec![2, 4, 8, 16]), 8);
        assert_eq!(Solution::min_operations(vec![1, 5]), 5);
        assert_eq!(Solution::min_operations(vec![2, 2]), 3);
        assert_eq!(Solution::min_operations(vec![4, 2, 5]), 6);
    }
}
