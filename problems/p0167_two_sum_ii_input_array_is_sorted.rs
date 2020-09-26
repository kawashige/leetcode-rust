pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            let num = target - numbers[i];
            for j in (i + 1)..numbers.len() {
                if num == numbers[j] {
                    return vec![i as i32 + 1, j as i32 + 1];
                } else if num < numbers[j] {
                    break;
                }
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0167() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 3], Solution::two_sum(vec![2, 3, 4], 6));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![-1, 0], -1));
    }
}
