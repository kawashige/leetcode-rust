pub struct Solution {}

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        fn recurse(nums: Vec<i32>, max: i32) -> Vec<i32> {
            let mut result = Vec::new();
            for num in nums {
                if max < num {
                    break;
                }
                result.push(num);
                result.append(&mut recurse(
                    (0..10)
                        .filter_map(|i| num.checked_mul(10).map(|j| j.checked_add(i)))
                        .map(|i| i.unwrap())
                        .collect(),
                    max,
                ))
            }
            result
        }
        recurse((1..10).collect(), n)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0386() {
        assert_eq!(
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9],
            Solution::lexical_order(13)
        );
        assert_eq!(vec![1, 2, 3], Solution::lexical_order(3));
    }
}
