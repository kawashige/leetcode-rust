pub struct Solution {}

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut l = 0_u64;
        for c in s.chars() {
            if c.is_numeric() {
                l *= c.to_digit(10).unwrap() as u64;
            } else {
                l += 1;
            }
        }

        let mut k = k as i64;
        for c in s.chars().rev() {
            k %= l as i64;
            if k == 0 && !c.is_numeric() {
                return c.to_string();
            }

            if c.is_numeric() {
                l /= c.to_digit(10).unwrap() as u64;
            } else {
                l -= 1;
            }
        }

        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day20() {
        assert_eq!(
            "l".to_string(),
            Solution::decode_at_index("yyuele72uthzyoeut7oyku2yqmghy5luy9qguc28ukav7an6a2bvizhph35t86qicv4gyeo6av7gerovv5lnw47954bsv2xruaej".to_string(), 123365626)
        );
        assert_eq!(
            "c".to_string(),
            Solution::decode_at_index("czjkk9elaqwiz7s6kgvl4gjixan3ky7jfdg3kyop3husw3fm289thisef8blt7a7zr5v5lhxqpntenvxnmlq7l34ay3jaayikjps".to_string(), 768077956)
        );
        assert_eq!(
            "x".to_string(),
            Solution::decode_at_index("n2f7x7bv4l".to_string(), 110)
        );
        assert_eq!(
            "a".to_string(),
            Solution::decode_at_index("a23".to_string(), 6)
        );
        assert_eq!(
            "x".to_string(),
            Solution::decode_at_index("cpmxv8ewnfk3xxcilcmm68d2ygc88daomywc3imncfjgtwj8nrxjtwhiem5nzqnicxzo248g52y72v3yujqpvqcssrofd99lkovg".to_string(), 480551547)
        );
        assert_eq!(
            "z".to_string(),
            Solution::decode_at_index("vzpp636m8y".to_string(), 2920)
        );
        assert_eq!(
            "o".to_string(),
            Solution::decode_at_index("leet2code3".to_string(), 10)
        );
        assert_eq!(
            "h".to_string(),
            Solution::decode_at_index("ha22".to_string(), 5)
        );
        assert_eq!(
            "a".to_string(),
            Solution::decode_at_index("a2345678999999999999999".to_string(), 1)
        );
    }
}
