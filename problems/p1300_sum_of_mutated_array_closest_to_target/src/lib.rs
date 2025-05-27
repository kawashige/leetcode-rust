pub struct Solution {}

impl Solution {
    pub fn diff(arr: &[i32], x: i32, target: i32) -> i32 {
        (target - arr.iter().map(|v| v.min(&x)).sum::<i32>()).abs()
    }

    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        arr.sort_unstable();

        let mut l = 0;
        let mut r = *arr.last().unwrap();

        while 3 <= r - l {
            let mid1 = l + (r - l) / 3;
            let mid2 = r - (r - l) / 3;
            let diff1 = Self::diff(&arr, mid1, target);
            let diff2 = Self::diff(&arr, mid2, target);
            if diff1 < diff2 {
                r = mid2;
            } else {
                l = mid1;
            }
        }

        (l..=r)
            .min_by_key(|x| Self::diff(&arr, *x, target))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1300() {
        assert_eq!(Solution::find_best_value(vec![4, 9, 3], 10), 3);
        assert_eq!(Solution::find_best_value(vec![2, 3, 5], 10), 5);
        assert_eq!(
            Solution::find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
            11361
        );
    }
}
