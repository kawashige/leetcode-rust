pub struct Solution {}

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 != 0 {
            return Default::default();
        }

        let mut result = Vec::with_capacity(changed.len() / 2);
        let mut count = [0; 100_001];
        changed.sort_unstable();

        for x in &changed {
            if 0 < count[*x as usize] {
                count[*x as usize] -= 1;
            } else {
                if count.len() <= *x as usize * 2 {
                    return Default::default();
                }
                result.push(*x);
                count[*x as usize * 2] += 1;
            }
        }

        if result.len() != changed.len() / 2 {
            Default::default()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2007() {
        assert_eq!(
            Solution::find_original_array(vec![1, 3, 4, 2, 6, 8]),
            vec![1, 3, 4]
        );
        assert_eq!(Solution::find_original_array(vec![6, 3, 0, 1]), vec![]);
        assert_eq!(Solution::find_original_array(vec![0, 0, 0, 0]), vec![0, 0]);
    }
}
