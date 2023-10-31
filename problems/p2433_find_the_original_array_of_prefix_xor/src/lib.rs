pub struct Solution {}

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![0; pref.len()];
        arr[0] = pref[0];

        for i in 1..pref.len() {
            arr[i] = pref[i - 1] ^ pref[i];
        }

        arr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2433() {
        assert_eq!(
            Solution::find_array(vec![5, 2, 0, 3, 1]),
            vec![5, 7, 2, 3, 2]
        );
        assert_eq!(Solution::find_array(vec![13]), vec![13]);
    }
}
