pub struct Solution {}

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned = banned.into_iter().fold([false; 10_001], |mut found, num| {
            found[num as usize] = true;
            found
        });

        let mut count = 0;
        let mut sum = 0;
        for i in 1..=n {
            if banned[i as usize] {
                continue;
            }
            if max_sum < sum + i {
                break;
            }
            sum += i;
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2554() {
        assert_eq!(Solution::max_count(vec![1, 6, 5], 5, 6), 2);
        assert_eq!(Solution::max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
        assert_eq!(Solution::max_count(vec![11], 7, 50), 7);
    }
}
