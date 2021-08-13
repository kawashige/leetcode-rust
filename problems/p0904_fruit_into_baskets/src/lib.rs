pub struct Solution {}

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count = vec![0; fruits.len() + 1];
        count[fruits[0] as usize] += 1;
        let mut current = 1;
        let mut r = 1;
        let mut i = 0;

        for j in 1..fruits.len() {
            if count[fruits[j] as usize] == 0 {
                current += 1;
                while current > 2 {
                    count[fruits[i] as usize] -= 1;
                    if count[fruits[i] as usize] == 0 {
                        current -= 1;
                    }
                    i += 1;
                }
            }
            count[fruits[j] as usize] += 1;
            r = r.max(j - i + 1);
        }

        r as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0904() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
        assert_eq!(
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
            5
        );
    }
}
