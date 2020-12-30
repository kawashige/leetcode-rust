pub struct Solution {}

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        if p.is_empty() {
            return 0;
        }

        let mut counts = [0; 26];
        let mut p_chars = p.chars();

        let mut prev = p_chars.next().unwrap() as usize - 0x61;
        counts[prev] = 1;

        let mut s = prev;
        let mut l = 1;
        for c in p_chars {
            let current = c as usize - 0x61;
            if (prev + 1) % 26 != current {
                s = current;
                l = 1;
            } else {
                l += 1;
            }

            let mut tmp_l = l;
            let mut tmp_s = s;
            while std::cmp::max(0, tmp_l - 26) < tmp_l {
                counts[tmp_s] = std::cmp::max(tmp_l, counts[tmp_s]);
                tmp_l -= 1;
                tmp_s = (tmp_s + 1) % 26;
            }

            prev = current;
        }

        println!("counts: {:?}", counts);

        counts.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0467() {
        assert_eq!(
            33475,
            Solution::find_substring_in_wrapround_string("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_string())
        );
        assert_eq!(
            1,
            Solution::find_substring_in_wrapround_string("a".to_string())
        );
        assert_eq!(
            2,
            Solution::find_substring_in_wrapround_string("cac".to_string())
        );
        assert_eq!(
            6,
            Solution::find_substring_in_wrapround_string("zab".to_string())
        );
        assert_eq!(
            6,
            Solution::find_substring_in_wrapround_string("zabzab".to_string())
        );
        assert_eq!(
            9,
            Solution::find_substring_in_wrapround_string("zabzabxy".to_string())
        );
        assert_eq!(
            0,
            Solution::find_substring_in_wrapround_string("".to_string())
        );
    }
}
