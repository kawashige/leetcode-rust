pub struct Solution {}

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        if let Some(e) = (0..nums.len())
            .filter(|i| nums[*i] == 0)
            .skip(k as usize)
            .next()
        {
            let mut s = 0;
            let mut max = e;

            for i in e..nums.len() {
                if nums[i] == 0 {
                    while nums[s] == 1 {
                        s += 1;
                    }
                    s += 1;
                }
                max = std::cmp::max(max, i - s + 1);
            }

            max as i32
        } else {
            nums.len() as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day29() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
        assert_eq!(
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
