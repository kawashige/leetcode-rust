pub struct Solution {}

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut count = vec![0; value as usize];
        for num in nums {
            if num < 0 {
                count[((value - num.abs() % value) % value) as usize] += 1;
            } else {
                count[(num % value) as usize] += 1;
            }
        }
        for i in 0.. {
            if count[(i % value) as usize] == 0 {
                return i;
            } else {
                count[(i % value) as usize] -= 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2598() {
        assert_eq!(
            Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5),
            4
        );
        assert_eq!(
            Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 7),
            2
        );
    }
}
