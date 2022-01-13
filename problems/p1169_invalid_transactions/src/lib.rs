pub struct Solution {}

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let transactions = transactions
            .into_iter()
            .map(|t| {
                let sp = t.split(",").collect::<Vec<_>>();
                (
                    sp[0].to_string(),
                    sp[1].parse::<i32>().unwrap(),
                    sp[2].parse::<usize>().unwrap(),
                    sp[3].to_string(),
                )
            })
            .collect::<Vec<_>>();

        let mut result = Vec::new();

        for i in 0..transactions.len() {
            let mut is_invalid = 1000 <= transactions[i].2;
            if !is_invalid {
                for j in 0..transactions.len() {
                    if i == j {
                        continue;
                    }
                    if transactions[i].0 == transactions[j].0
                        && (transactions[i].1 - 60..=transactions[i].1 + 60)
                            .contains(&transactions[j].1)
                        && transactions[i].3 != transactions[j].3
                    {
                        is_invalid = true;
                        break;
                    }
                }
            }

            if is_invalid {
                result.push(format!(
                    "{},{},{},{}",
                    transactions[i].0, transactions[i].1, transactions[i].2, transactions[i].3
                ));
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1169() {
        assert_eq!(
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,beijing".to_string()
            ]),
            vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,beijing".to_string()
            ]
        );
        assert_eq!(
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,1200,mtv".to_string()
            ]),
            vec!["alice,50,1200,mtv".to_string()]
        );
        assert_eq!(
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "bob,50,1200,mtv".to_string()
            ]),
            vec!["bob,50,1200,mtv".to_string()]
        );
    }
}
