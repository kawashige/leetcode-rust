pub struct Solution {}

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = [false; 101];
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        for n in nums {
            seen[n as usize] = true;
            min = min.min(n);
            max = max.max(n)
        }
        (min..=max).filter(|i| !seen[*i as usize]).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3731() {
        assert_eq!(Solution::find_missing_elements(vec![1, 4, 2, 5]), vec![3]);
        assert_eq!(Solution::find_missing_elements(vec![7, 8, 6, 9]), vec![]);
        assert_eq!(Solution::find_missing_elements(vec![5, 1]), vec![2, 3, 4]);
    }
}

fn main() {}
