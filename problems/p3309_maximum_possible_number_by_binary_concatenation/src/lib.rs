pub struct Solution {}

impl Solution {
    pub fn max_good_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                for k in 0..nums.len() {
                    if i == k || j == k {
                        continue;
                    }
                    let b = format!("{:b}{:b}{:b}", nums[i], nums[j], nums[k]);
                    let num = usize::from_str_radix(&b, 2).unwrap();
                    result = result.max(num);
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3309() {
        assert_eq!(Solution::max_good_number(vec![1, 2, 3]), 30);
        assert_eq!(Solution::max_good_number(vec![2, 8, 16]), 1296);
    }
}
