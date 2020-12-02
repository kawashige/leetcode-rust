pub struct Solution {}

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let hexes = "0123456789abcdef".chars().collect::<Vec<char>>();
        let mut n = if num < 0 { !num.abs() + 1 } else { num };

        let mut results = Vec::new();
        for _ in 0..8 {
            results.push(hexes[(0..16).find(|i| (n & 15) == *i).unwrap() as usize]);
            n >>= 4;
        }
        while 1 < results.len() && results.last().unwrap() == &'0' {
            results.pop();
        }

        results.into_iter().rev().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0405() {
        assert_eq!("1a", Solution::to_hex(26));
        assert_eq!("ffffffff", Solution::to_hex(-1));
    }
}
