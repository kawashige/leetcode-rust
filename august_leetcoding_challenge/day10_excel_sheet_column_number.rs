pub struct Solution {}

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        use std::collections::HashMap;
        let dic = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .zip(1..28)
            .collect::<HashMap<_, _>>();
        println!("{:?}", dic);
        let mut chars = s.chars().collect::<Vec<char>>();
        chars.reverse();
        let mut result = 0;
        for i in 0..chars.len() {
            println!("i: {:?}, result: {:?}, c: {:?}", i, result, chars[i]);
            result += (26 as i32).pow(i as u32) * dic.get(&chars[i]).unwrap();
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day10() {
        assert_eq!(1, Solution::title_to_number("A".to_string()));
        assert_eq!(28, Solution::title_to_number("AB".to_string()));
        assert_eq!(701, Solution::title_to_number("ZY".to_string()));
        assert_eq!(2147483647, Solution::title_to_number("FXSHRXW".to_string()));
    }
}
