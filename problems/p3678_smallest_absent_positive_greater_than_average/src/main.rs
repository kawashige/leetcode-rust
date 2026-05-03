pub struct Solution {}

impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let average = nums.iter().sum::<i32>() / nums.len() as i32;
        for i in average.max(0) + 1.. {
            if !nums.contains(&i) {
                return i as i32;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3678() {
        assert_eq!(Solution::smallest_absent(vec![3, 5]), 6);
        assert_eq!(Solution::smallest_absent(vec![-1, 1, 2]), 3);
        assert_eq!(Solution::smallest_absent(vec![4, -1]), 2);
    }
}

fn main() {}
