pub struct Solution {}

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        nums.sort_unstable();
        let mut count = 0;
        for i in 0..(nums.len() - 2) {
            if nums[i] == 0 {
                continue;
            }
            let mut k = i + 2;
            for j in (i + 1)..(nums.len() - 1) {
                while k < nums.len() && nums[i] + nums[j] > nums[k] {
                    k += 1;
                }
                count += k - j - 1;
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0611() {
        assert_eq!(Solution::triangle_number(vec![1]), 0);
        assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    }
}
