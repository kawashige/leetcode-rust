pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().rev().fold(0, |acc, x| match x {
            'I' => {
                if acc < 5 {
                    acc + 1
                } else {
                    acc - 1
                }
            }
            'V' => acc + 5,
            'X' => {
                if acc < 50 {
                    acc + 10
                } else {
                    acc - 10
                }
            }
            'L' => acc + 50,
            'C' => {
                if acc < 500 {
                    acc + 100
                } else {
                    acc - 100
                }
            }
            'D' => acc + 500,
            'M' => acc + 1000,
            _ => acc,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_13() {
        let ret = Solution::roman_to_int("MCMXCIV".to_string());
        assert_eq!(1994, ret)
    }
}
