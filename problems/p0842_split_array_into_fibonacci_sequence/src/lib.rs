pub struct Solution {}

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        for i in 1..(num.len() - 1) {
            let mut result = Vec::new();

            if num.starts_with("0") && i > 1 {
                break;
            }

            if let Ok(num1) = num[..i].parse::<i32>() {
                result.push(num1);
            } else {
                break;
            }

            for j in (i + 1)..num.len() {
                if num[i..j].starts_with("0") && j > i + 1 {
                    break;
                }

                if let Ok(num2) = num[i..j].parse::<i32>() {
                    result.push(num2);
                } else {
                    break;
                }

                let mut k = j;
                while k < num.len() {
                    let x = result[result.len() - 2] + result[result.len() - 1];
                    if num[k..].starts_with(&format!("{}", x)) {
                        result.push(x);
                        k += x.to_string().len();
                    } else {
                        break;
                    }
                }

                if k == num.len() {
                    return result;
                }
                result = vec![result[0]];
            }
        }

        Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_00842() {
        assert_eq!(
            Solution::split_into_fibonacci("123456579".to_string()),
            vec![123, 456, 579]
        );
        assert_eq!(
            Solution::split_into_fibonacci("11235813".to_string()),
            vec![1, 1, 2, 3, 5, 8, 13]
        );
        assert_eq!(
            Solution::split_into_fibonacci("112358130".to_string()),
            vec![]
        );
        assert_eq!(
            Solution::split_into_fibonacci("112358130".to_string()),
            vec![]
        );
        assert_eq!(
            Solution::split_into_fibonacci("1101111".to_string()),
            vec![11, 0, 11, 11]
        );
    }
}
