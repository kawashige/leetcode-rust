pub struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.len() == 1 {
            return 0;
        }
        let mut max = b'a';
        let mut min = b'z';
        let mut is_sorted = true;
        let mut count = [0; 26];
        for i in 0..s.len() {
            max = max.max(s.as_bytes()[i]);
            min = min.min(s.as_bytes()[i]);
            count[(s.as_bytes()[i] - b'a') as usize] += 1;
            if 0 < i && s.as_bytes()[i] < s.as_bytes()[i - 1] {
                is_sorted = false;
            }
        }

        if is_sorted {
            0
        } else if s.len() == 2 {
            -1
        } else if s.as_bytes()[0] == max
            && s.as_bytes()[s.len() - 1] == min
            && count[(min - b'a') as usize] == 1
            && count[(max - b'a') as usize] == 1
        {
            3
        } else if s.as_bytes()[0] == min || s.as_bytes()[s.len() - 1] == max {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3863() {
        assert_eq!(Solution::min_operations("dog".to_string()), 1);
        assert_eq!(Solution::min_operations("card".to_string()), 2);
        assert_eq!(Solution::min_operations("gf".to_string()), -1);
    }
}

fn main() {}
