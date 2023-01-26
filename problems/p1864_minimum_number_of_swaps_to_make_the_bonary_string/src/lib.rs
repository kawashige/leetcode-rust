pub struct Solution {}

impl Solution {
    pub fn count(s: &str, start: i32) -> i32 {
        s.as_bytes()
            .iter()
            .enumerate()
            .filter(|(i, b)| {
                *b - b'0'
                    != if start == 0 {
                        *i as u8 % 2
                    } else {
                        (*i as u8 + 1) % 2
                    }
            })
            .count() as i32
            / 2
    }

    pub fn min_swaps(s: String) -> i32 {
        let counts: [i32; 2] = s.as_bytes().iter().fold([0; 2], |mut counts, b| {
            counts[(b - b'0') as usize] += 1;
            counts
        });
        if 1 < (counts[0] - counts[1]).abs() {
            return -1;
        }

        if counts[0] == counts[1] {
            Self::count(&s, 0).min(Self::count(&s, 1))
        } else if counts[1] < counts[0] {
            Self::count(&s, 0)
        } else {
            Self::count(&s, 1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1864() {
        assert_eq!(Solution::min_swaps("111000".to_string()), 1);
        assert_eq!(Solution::min_swaps("010".to_string()), 0);
        assert_eq!(Solution::min_swaps("1110".to_string()), -1);
    }
}
