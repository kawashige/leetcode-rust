pub struct Solution {}

impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        let mut num = num.chars().collect::<Vec<_>>();

        if let Some(i) = (0..num.len())
            .find(|i| ((num[*i] as u8 - b'0') as i32) < change[(num[*i] as u8 - b'0') as usize])
        {
            let mut j = i;
            while j < num.len()
                && (num[j] as u8 - b'0') as i32 <= change[(num[j] as u8 - b'0') as usize]
            {
                num[j] = (change[(num[j] as u8 - b'0') as usize] as u8 + b'0') as char;
                j += 1;
            }
        };

        num.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1946() {
        assert_eq!(
            Solution::maximum_number("334111".to_string(), vec![0, 9, 2, 3, 3, 2, 5, 5, 5, 5]),
            "334999".to_string()
        );
        assert_eq!(
            Solution::maximum_number("132".to_string(), vec![9, 8, 5, 0, 3, 6, 4, 2, 6, 8]),
            "832".to_string()
        );
        assert_eq!(
            Solution::maximum_number("021".to_string(), vec![9, 4, 3, 5, 7, 2, 1, 9, 0, 6]),
            "934".to_string()
        );
        assert_eq!(
            Solution::maximum_number("5".to_string(), vec![1, 4, 7, 5, 3, 2, 5, 6, 9, 4]),
            "5".to_string()
        );
    }
}
