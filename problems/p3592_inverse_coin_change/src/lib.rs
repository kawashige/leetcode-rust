pub struct Solution {}

impl Solution {
    pub fn find_coins(num_ways: Vec<i32>) -> Vec<i32> {
        let mut num_ways = num_ways;
        let mut result = Vec::new();
        while let Some(i) = (0..num_ways.len()).find(|i| num_ways[*i] == 1) {
            result.push(i as i32 + 1);
            for j in (i + 1..num_ways.len()).rev() {
                num_ways[j] -= num_ways[j - i - 1];
                if num_ways[j] < 0 {
                    return Default::default();
                }
            }
            num_ways[i] = 0;
        }

        if num_ways.into_iter().all(|w| w == 0) {
            result
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3592() {
        assert_eq!(
            Solution::find_coins(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5]),
            vec![2, 4, 6]
        );
        assert_eq!(Solution::find_coins(vec![1, 2, 2, 3, 4]), vec![1, 2, 5]);
        assert_eq!(Solution::find_coins(vec![1, 2, 3, 4, 15]), vec![]);
    }
}
