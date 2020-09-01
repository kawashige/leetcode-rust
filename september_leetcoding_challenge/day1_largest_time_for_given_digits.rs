pub struct Solution {}

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let mut sorted = a.clone();
        sorted.sort();
        sorted.reverse();
        for i in 0..sorted.len() {
            if 2 < sorted[i] {
                continue;
            }
            for j in 0..sorted.len() {
                if i == j || (sorted[i] == 2 && 3 < sorted[j]) {
                    continue;
                }
                for k in 0..sorted.len() {
                    if k == i || k == j || 5 < sorted[k] {
                        continue;
                    }
                    for l in 0..sorted.len() {
                        if l == k || l == i || l == j {
                            continue;
                        }
                        return format!("{}{}:{}{}", sorted[i], sorted[j], sorted[k], sorted[l]);
                    }
                }
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day1() {
        assert_eq!(
            "23:41",
            Solution::largest_time_from_digits(vec![1, 2, 3, 4])
        );
        assert_eq!(
            "09:59",
            Solution::largest_time_from_digits(vec![0, 9, 5, 9])
        );
        assert_eq!("", Solution::largest_time_from_digits(vec![5, 5, 5, 5]));
    }
}
