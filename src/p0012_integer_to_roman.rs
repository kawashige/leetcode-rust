pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman_c = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let roman_v = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

        let mut results = Vec::new();
        let mut n = num;
        for i in 0..roman_c.len() {
            for _ in 0..(n / roman_v[i]) {
                results.push(roman_c[i]);
            }
            n %= roman_v[i];
        }

        results.concat()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0012() {
        assert_eq!("III", Solution::int_to_roman(3));
        assert_eq!("IV", Solution::int_to_roman(4));
        assert_eq!("LVIII", Solution::int_to_roman(58));
        assert_eq!("IX", Solution::int_to_roman(9));
    }
}
