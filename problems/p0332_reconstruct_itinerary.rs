pub struct Solution {}

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        fn recurse(tickets: &Vec<Vec<String>>, routes: Vec<usize>, next: &str) -> Vec<usize> {
            if routes.len() == tickets.len() {
                return routes;
            }
            for i in 0..tickets.len() {
                if tickets[i][0] != next || routes.contains(&i) {
                    continue;
                }
                let mut new_routes = routes.clone();
                new_routes.push(i);
                let result = recurse(tickets, new_routes, &tickets[i][1]);
                if result.len() == tickets.len() {
                    return result;
                }
            }
            routes
        }

        let mut tickets = tickets;
        tickets.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let routes = recurse(&tickets, Vec::new(), "JFK");
        routes
            .iter()
            .map(|r| tickets[*r][0].clone())
            .chain(std::iter::once(tickets[*routes.last().unwrap()][1].clone()))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0332() {
        assert_eq!(
            vec![
                "JFK".to_string(),
                "MUC".to_string(),
                "LHR".to_string(),
                "SFO".to_string(),
                "SJC".to_string()
            ],
            Solution::find_itinerary(vec![
                vec!["MUC".to_string(), "LHR".to_string()],
                vec!["JFK".to_string(), "MUC".to_string()],
                vec!["SFO".to_string(), "SJC".to_string()],
                vec!["LHR".to_string(), "SFO".to_string()]
            ])
        );

        assert_eq!(
            vec![
                "JFK".to_string(),
                "ATL".to_string(),
                "JFK".to_string(),
                "SFO".to_string(),
                "ATL".to_string(),
                "SFO".to_string(),
            ],
            Solution::find_itinerary(vec![
                vec!["JFK".to_string(), "SFO".to_string()],
                vec!["JFK".to_string(), "ATL".to_string()],
                vec!["SFO".to_string(), "ATL".to_string()],
                vec!["ATL".to_string(), "JFK".to_string()],
                vec!["ATL".to_string(), "SFO".to_string()]
            ])
        );
    }
}
