pub struct Solution {}

impl Solution {
    pub fn recurse(
        is_break: &mut Vec<bool>,
        time: i32,
        x: i32,
        k: i32,
        strength: &[i32],
        min_time: &mut i32,
    ) {
        if is_break.iter().all(|b| *b) {
            *min_time = std::cmp::min(*min_time, time);
            return;
        }

        for i in 0..is_break.len() {
            if is_break[i] {
                continue;
            }
            is_break[i] = true;
            Self::recurse(
                is_break,
                time + (strength[i] + x - 1) / x,
                x + k,
                k,
                strength,
                min_time,
            );
            is_break[i] = false;
        }
    }

    pub fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
        let mut min_time = std::i32::MAX;

        Self::recurse(
            &mut vec![false; strength.len()],
            0,
            1,
            k,
            &strength,
            &mut min_time,
        );

        min_time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3376() {
        assert_eq!(Solution::find_minimum_time(vec![3, 4, 1], 1), 4);
        assert_eq!(Solution::find_minimum_time(vec![2, 5, 4,], 2), 5);
    }
}
