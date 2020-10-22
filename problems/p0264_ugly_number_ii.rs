pub struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n < 0 {
            0
        } else {
            let mut nums: Vec<i32> = vec![1];
            let mut current = [0usize; 3];
            let muls = [2, 3, 5];
            while nums.len() < n as usize {
                let (index, next) = current
                    .iter()
                    .enumerate()
                    .map(|(i, c)| (i, nums[*c] * muls[i]))
                    .min_by_key(|x| x.1)
                    .unwrap();
                nums.push(next);
                current[index] += 1;
                for i in 0..3 {
                    if i != index && nums[current[i]] * muls[i] == next {
                        current[i] += 1;
                    }
                }
            }
            *nums.last().unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0264() {
        assert_eq!(1, Solution::nth_ugly_number(1));
        assert_eq!(12, Solution::nth_ugly_number(10));
        assert_eq!(2123366400, Solution::nth_ugly_number(1690));
    }
}
