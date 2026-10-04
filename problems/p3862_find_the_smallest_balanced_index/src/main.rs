pub struct Solution {}

impl Solution {
    pub fn smallest_balanced_index(nums: Vec<i32>) -> i32 {
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }
        let mut product: Vec<i64> = vec![1; nums.len() + 1];
        for i in (0..nums.len()).rev() {
            product[i] = product[i + 1].saturating_mul(nums[i] as i64);
        }

        (0..nums.len() as i32)
            .find(|i| sum[*i as usize] == product[*i as usize + 1])
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3862() {
        assert_eq!(
            Solution::smallest_balanced_index(vec![
                813, 974, 946, 966, 915, 924, 812, 1000, 891, 875, 989, 656, 991, 806, 818, 999,
                971, 276, 923, 997, 992, 943, 983, 811, 909, 990, 924, 991, 726, 818, 969, 690,
                996, 784, 992, 949, 915, 931, 932, 821, 699, 688, 712, 805, 849, 489, 406, 482,
                777, 974, 479, 237, 963, 903, 957, 995, 814, 864, 832, 889, 936, 467, 831, 970,
                757, 646, 962, 987, 885, 924, 918, 710, 763, 839, 860, 888, 971, 994, 339, 253,
                564, 759, 68, 747, 797, 716, 939, 987, 68, 953, 1000, 298, 10, 1, 1, 1, 1, 1, 48,
                1, 77, 2
            ]),
            91
        );
        assert_eq!(Solution::smallest_balanced_index(vec![2, 1, 2]), 1);
        assert_eq!(Solution::smallest_balanced_index(vec![2, 8, 2, 2, 5]), 2);
        assert_eq!(Solution::smallest_balanced_index(vec![1]), -1);
    }
}

fn main() {}
