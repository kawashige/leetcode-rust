pub struct Solution {}

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let num1 = num.into_iter().rev().collect::<Vec<_>>();
        let num2 = k
            .to_string()
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();

        let mut r = Vec::new();
        let mut c = 0;

        for i in 0..num1.len().max(num2.len()) {
            let mut d = c;
            if i < num1.len() {
                d += num1[i];
            }
            if i < num2.len() {
                d += num2[i]
            }
            r.push(d % 10);
            c = d / 10;
        }
        if c > 0 {
            r.push(c);
        }

        r.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0989() {
        assert_eq!(
            Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::add_to_array_form(vec![2, 7, 4], 181),
            vec![4, 5, 5]
        );
        assert_eq!(
            Solution::add_to_array_form(vec![2, 1, 5], 806),
            vec![1, 0, 2, 1]
        );
        assert_eq!(
            Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
