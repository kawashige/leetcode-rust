use std::collections::BTreeSet;
pub struct Solution {}

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut x_pows = vec![1];
        if x > 1 {
            while x_pows.last().unwrap() * x < bound {
                x_pows.push(x_pows.last().unwrap() * x);
            }
        }
        let mut y_pows = vec![1];
        if y > 1 {
            while y_pows.last().unwrap() * y < bound {
                y_pows.push(y_pows.last().unwrap() * y);
            }
        }

        let mut set = BTreeSet::new();
        for x_pow in x_pows {
            for y_pow in &y_pows {
                if x_pow + y_pow > bound {
                    break;
                }
                set.insert(x_pow + y_pow);
            }
        }

        set.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day30() {
        assert_eq!(Solution::powerful_integers(1, 1, 0), vec![]);
        assert_eq!(
            Solution::powerful_integers(2, 3, 10),
            vec![2, 3, 4, 5, 7, 9, 10]
        );

        assert_eq!(
            Solution::powerful_integers(3, 5, 15),
            vec![2, 4, 6, 8, 10, 14]
        );
    }
}
