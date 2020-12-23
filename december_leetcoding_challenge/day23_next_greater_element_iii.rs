pub struct Solution {}

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .rev()
            .collect::<Vec<i64>>();
        let l = digits.len();

        for i in 1..l {
            if let Some(j) = (0..i)
                .filter(|j| digits[i] < digits[*j])
                .min_by(|a, b| digits[*a].cmp(&digits[*b]))
            {
                digits.swap(i, j);
                let mut need_sort = digits.drain(..i).collect::<Vec<i64>>();
                digits.reverse();
                need_sort.reverse();
                need_sort.sort();

                return digits
                    .into_iter()
                    .chain(need_sort.into_iter())
                    .map(|d| d.to_string())
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap_or(-1);
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day23() {
        assert_eq!(-1, Solution::next_greater_element(2_147_483_647i32));
        assert_eq!(419238, Solution::next_greater_element(418932));
        assert_eq!(-1, Solution::next_greater_element(1));
        assert_eq!(21, Solution::next_greater_element(12));
        assert_eq!(-1, Solution::next_greater_element(21));
    }
}
