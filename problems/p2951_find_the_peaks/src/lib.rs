pub struct Solution {}

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        (1..mountain.len() as i32 - 1)
            .filter(|i| {
                mountain[*i as usize - 1] < mountain[*i as usize]
                    && mountain[*i as usize + 1] < mountain[*i as usize]
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2951() {
        assert_eq!(Solution::find_peaks(vec![2, 4, 4]), vec![]);
        assert_eq!(Solution::find_peaks(vec![1, 4, 3, 8, 5]), vec![1, 3]);
    }
}
