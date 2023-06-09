pub struct Solution {}

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let remains =
            (n as usize + rolls.len()) * mean as usize - rolls.iter().sum::<i32>() as usize;
        let mut result = Vec::with_capacity(n as usize);

        if remains < n as usize || n as usize * 6 < remains {
            return result;
        }

        let base = (remains / n as usize) as i32;
        let r = remains % n as usize;

        for i in 0..n as usize {
            result.push(base + if i < r { 1 } else { 0 });
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2028() {
        assert_eq!(Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]);
        assert_eq!(
            Solution::missing_rolls(vec![1, 5, 6], 3, 4),
            vec![3, 2, 2, 2]
        );
        assert_eq!(Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4), vec![]);
    }
}
