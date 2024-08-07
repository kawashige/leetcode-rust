pub struct Solution {}

impl Solution {
    pub fn parse_under_thousand(num: i32) -> String {
        const DIGITS: [&str; 10] = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ];
        let mut result = String::new();
        if 100 <= num {
            result += DIGITS[(num / 100) as usize];
            result += " Hundred";
        }
        if 100 <= num && 0 < num % 100 {
            result += " ";
        }
        result + &Self::parse_under_hundred(num % 100)
    }

    pub fn parse_under_hundred(num: i32) -> String {
        const DIGITS: [&str; 10] = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ];
        const TWENTY_TO_NINETEEN: [&str; 10] = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        if num < 10 {
            DIGITS[(num % 10) as usize].to_string()
        } else if num < 20 {
            (match num {
                10 => "Ten",
                11 => "Eleven",
                12 => "Twelve",
                13 => "Thirteen",
                14 => "Fourteen",
                15 => "Fifteen",
                16 => "Sixteen",
                17 => "Seventeen",
                18 => "Eighteen",
                19 => "Nineteen",
                _ => unreachable!(),
            })
            .to_string()
        } else {
            TWENTY_TO_NINETEEN[(num / 10) as usize].to_string()
                + if num % 10 == 0 { "" } else { " " }
                + DIGITS[(num % 10) as usize]
        }
    }
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        let mut result = String::new();
        let mut num = num;

        if 0 < num / 1_000_000_000 {
            result += if result.is_empty() { "" } else { " " };
            result += &Self::parse_under_thousand(num / 1_000_000_000);
            result += " Billion";
            num %= 1_000_000_000;
        }
        if 0 < num / 1_000_000 {
            result += if result.is_empty() { "" } else { " " };
            result += &Self::parse_under_thousand(num / 1_000_000);
            result += " Million";
            num %= 1_000_000;
        }
        if 0 < num / 1_000 {
            result += if result.is_empty() { "" } else { " " };
            result += &Self::parse_under_thousand(num / 1_000);
            result += " Thousand";
            num %= 1_000;
        }
        if 0 < num {
            result += if result.is_empty() { "" } else { " " };
            result += &Self::parse_under_thousand(num);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0273() {
        assert_eq!(
            Solution::number_to_words(123),
            "One Hundred Twenty Three".to_string()
        );
        assert_eq!(
            Solution::number_to_words(12_345),
            "Twelve Thousand Three Hundred Forty Five".to_string()
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string()
        );
    }
}
