pub struct Solution {}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut placed = vec![false; baskets.len()];
        let mut result = 0;

        for i in 0..fruits.len() {
            let mut unplaced = true;
            for j in 0..baskets.len() {
                if placed[j] || baskets[j] < fruits[i] {
                    continue;
                }
                placed[j] = true;
                unplaced = false;
                break;
            }
            if unplaced {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3477() {
        assert_eq!(
            Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]),
            1
        );
        assert_eq!(
            Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]),
            0
        );
    }
}
