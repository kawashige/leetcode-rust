pub struct Solution {}

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut l = 0;
        while l + 1 < arr.len() && arr[l] <= arr[l + 1] {
            l += 1;
        }

        if l == arr.len() - 1 {
            return 0;
        }

        let mut r = arr.len() - 1;
        while 0 < r && arr[r - 1] <= arr[r] {
            r -= 1;
        }

        let count = r - l - 1;
        let mut min = count + l + 1;

        let mut j = arr.len();
        let mut tmp = 0;
        for i in (0..=l).rev() {
            while r <= j - 1 && arr[i] <= arr[j - 1] {
                j -= 1;
                tmp += 1;
            }
            min = min.min(count + l - i + arr.len() - r - tmp);
        }

        min as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1574() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![16, 10, 0, 3, 22, 1, 14, 7, 1, 12, 15]),
            8
        );
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
            3
        );
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]),
            4
        );
        assert_eq!(Solution::find_length_of_shortest_subarray(vec![1, 2, 3]), 0);
    }
}
