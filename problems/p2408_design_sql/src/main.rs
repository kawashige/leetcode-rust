use std::collections::HashMap;

struct SQL {
    tables: HashMap<String, TABLE>,
}

struct TABLE {
    column_number: usize,
    rows: Vec<Vec<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SQL {
    fn new(names: Vec<String>, columns: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for i in 0..names.len() {
            map.insert(
                names[i].to_string(),
                TABLE {
                    column_number: columns[i] as usize,
                    rows: Vec::new(),
                },
            );
        }
        Self { tables: map }
    }

    fn ins(&mut self, name: String, row: Vec<String>) -> bool {
        if let Some(t) = self.tables.get_mut(&name) {
            if t.column_number != row.len() {
                return false;
            }
            t.rows.push(row);
            true
        } else {
            false
        }
    }

    fn rmv(&mut self, name: String, row_id: i32) {
        if let Some(t) = self.tables.get_mut(&name) {
            let row_id = row_id as usize - 1;
            if row_id < t.rows.len() {
                t.rows[row_id].clear();
            }
        }
    }

    fn sel(&self, name: String, row_id: i32, column_id: i32) -> String {
        let null_value = "<null>".to_string();
        if let Some(t) = self.tables.get(&name) {
            let row_id = row_id as usize - 1;
            let column_id = column_id as usize - 1;
            if t.rows.len() <= row_id || t.rows[row_id].len() <= column_id {
                return null_value;
            }
            t.rows[row_id][column_id].clone()
        } else {
            null_value
        }
    }

    fn exp(&self, name: String) -> Vec<String> {
        if let Some(t) = self.tables.get(&name) {
            t.rows
                .iter()
                .enumerate()
                .filter_map(|(i, r)| {
                    if r.is_empty() {
                        None
                    } else {
                        Some(format!("{},{}", i + 1, r.join(",")))
                    }
                })
                .collect()
        } else {
            Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2408() {
        let mut obj = SQL::new(
            vec!["one".to_string(), "two".to_string(), "three".to_string()],
            vec![2, 3, 1],
        );
        assert!(obj.ins(
            "two".to_string(),
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string()
            ]
        ));
        assert_eq!(obj.sel("two".to_string(), 1, 3), "third".to_string());
        assert!(obj.ins(
            "two".to_string(),
            vec![
                "fourth".to_string(),
                "fifth".to_string(),
                "sixth".to_string()
            ]
        ));
        assert_eq!(
            obj.exp("two".to_string()),
            vec![
                "1,first,second,third".to_string(),
                "2,fourth,fifth,sixth".to_string()
            ]
        );
        obj.rmv("two".to_string(), 1);
        assert_eq!(obj.sel("two".to_string(), 2, 2), "fifth".to_string());
        assert_eq!(
            obj.exp("two".to_string()),
            vec!["2,fourth,fifth,sixth".to_string()]
        );
    }
}

fn main() {}
