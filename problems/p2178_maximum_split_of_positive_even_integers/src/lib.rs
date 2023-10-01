pub struct Solution {}

impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        if final_sum % 2 == 1 {
            return Default::default();
        }

        let mut result = Vec::new();
        let mut remains = final_sum;
        let mut current = 2;

        while current < remains - current {
            result.push(current);
            remains -= current;
            current += 2;
        }

        if 0 < remains {
            result.push(remains);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2178() {
        assert_eq!(Solution::maximum_even_split(12), vec![2, 4, 6]);
        assert_eq!(Solution::maximum_even_split(7), vec![]);
        assert_eq!(Solution::maximum_even_split(28), vec![2, 4, 6, 16]);
    }
}
