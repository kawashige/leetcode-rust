pub struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.len() <= k as usize {
            return s.len() as i32;
        }

        let chars = s.chars().collect::<Vec<char>>();

        let mut max = 0;
        let mut counts = [0; 26];
        let mut i = 0;
        let mut j = i;
        counts[chars[i] as usize - 0x41] += 1;
        while i < chars.len() && j < chars.len() {
            // println!("i: {}, j: {}, counts: {:?}", i, j, counts);
            let mut tmp_max = *counts.iter().max().unwrap();
            while j + 1 < chars.len() && j - i + 1 <= tmp_max + k as usize {
                j += 1;
                counts[chars[j] as usize - 0x41] += 1;
                tmp_max = std::cmp::max(tmp_max, counts[chars[j] as usize - 0x41]);
            }
            if j - i + 1 <= tmp_max + k as usize {
                max = std::cmp::max(max, j - i + 1);
            } else {
                max = std::cmp::max(max, j - 1 - i + 1);
            }
            if chars.len() <= j + 1 {
                break;
            }
            max = std::cmp::max(max, j - 1 - i + 1);
            while j - i + 1 > counts.iter().max().unwrap() + k as usize {
                counts[chars[i] as usize - 0x41] -= 1;
                i += 1;
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0424() {
        assert_eq!(0, Solution::character_replacement("".to_string(), 1));
        assert_eq!(3, Solution::character_replacement("ACC".to_string(), 3));
        assert_eq!(4, Solution::character_replacement("ABAB".to_string(), 2));
        assert_eq!(
            4,
            Solution::character_replacement("ABAABABB".to_string(), 1)
        );
    }
}
