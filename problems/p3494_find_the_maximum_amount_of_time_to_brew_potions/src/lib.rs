pub struct Solution {}

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let mut time = vec![0; skill.len() + 1];
        for m in mana {
            let mut offset = 0;
            time[0] = time[1];
            for i in 0..skill.len() {
                time[i + 1] = time[i] + skill[i] as i64 * m as i64;
                if i < skill.len() - 1 {
                    offset = offset.max(time[i + 2] - time[i + 1]);
                }
            }
            if 0 < offset {
                for i in 0..time.len() {
                    time[i] += offset;
                }
            }
        }
        *time.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3494() {
        assert_eq!(Solution::min_time(vec![3, 5, 3, 9], vec![1, 10, 7, 3]), 293);
        assert_eq!(Solution::min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]), 110);
        assert_eq!(Solution::min_time(vec![1, 1, 1], vec![1, 1, 1]), 5);
        assert_eq!(Solution::min_time(vec![1, 2, 3, 4], vec![1, 2]), 21);
    }
}
