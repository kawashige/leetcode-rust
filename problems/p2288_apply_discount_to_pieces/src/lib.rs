pub struct Solution {}

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        sentence
            .split_ascii_whitespace()
            .map(|s| {
                if s.starts_with('$') && s[1..].parse::<usize>().is_ok() {
                    let d = s[1..].parse::<usize>().unwrap();
                    let price = 100 * d - d * discount as usize;
                    if price < 10 {
                        format!("$0.0{}", price)
                    } else if price < 100 {
                        format!("$0.{}", price)
                    } else {
                        let p = price.to_string();
                        format!(
                            "${}.{}",
                            p[..p.len() - 2].to_string(),
                            p[p.len() - 2..].to_string()
                        )
                    }
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2288() {
        assert_eq!(
            Solution::discount_prices("there are $1 $2 and 5$ candies in the shop".to_string(), 50),
            "there are $0.50 $1.00 and 5$ candies in the shop".to_string()
        );
        assert_eq!(
            Solution::discount_prices("1 2 $3 4 $5 $6 7 8$ $9 $10$".to_string(), 100),
            "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$".to_string()
        );
    }
}
