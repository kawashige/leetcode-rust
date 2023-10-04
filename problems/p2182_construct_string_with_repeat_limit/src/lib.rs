pub struct Solution {}

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut count = s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });

        let mut result = String::new();
        let mut last = 26;
        let mut repeat = 0;
        let mut changed = true;

        while changed {
            changed = false;
            for i in (0..26).rev() {
                if 0 < count[i] && (repeat < repeat_limit || last != i) {
                    result.push((i as u8 + b'a') as char);
                    count[i] -= 1;
                    if last == i {
                        repeat += 1;
                    } else {
                        repeat = 1;
                    }
                    last = i;
                    changed = true;
                    break;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2182() {
        assert_eq!(
            Solution::repeat_limited_string("cczazcc".to_string(), 3),
            "zzcccac".to_string()
        );
        assert_eq!(
            Solution::repeat_limited_string("aababab".to_string(), 2),
            "bbabaa".to_string()
        );
    }
}
