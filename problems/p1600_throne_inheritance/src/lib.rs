use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    king: String,
    relationship: HashMap<String, Vec<String>>,
    deaths: HashSet<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ThroneInheritance {
    fn new(kingName: String) -> Self {
        let mut relationship = HashMap::new();
        relationship.insert(kingName.clone(), Vec::new());
        Self {
            king: kingName,
            relationship,
            deaths: HashSet::new(),
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        (*self.relationship.get_mut(&parent_name).unwrap()).push(child_name.clone());
        self.relationship.insert(child_name, Vec::new());
    }

    fn death(&mut self, name: String) {
        self.deaths.insert(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut order = Vec::new();

        self.recurse(&self.king, &mut order);

        order
    }

    fn recurse(&self, name: &str, order: &mut Vec<String>) {
        if !self.deaths.contains(name) {
            order.push(name.to_string());
        }
        for child in &self.relationship[name] {
            Self::recurse(&self, child, order);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1600() {
        let mut obj = ThroneInheritance::new("king".to_string());
        obj.birth("king".to_string(), "andy".to_string());
        obj.birth("king".to_string(), "bob".to_string());
        obj.birth("king".to_string(), "catherine".to_string());
        obj.birth("andy".to_string(), "matthew".to_string());
        obj.birth("bob".to_string(), "alex".to_string());
        obj.birth("bob".to_string(), "asha".to_string());
        assert_eq!(
            obj.get_inheritance_order(),
            vec![
                "king".to_string(),
                "andy".to_string(),
                "matthew".to_string(),
                "bob".to_string(),
                "alex".to_string(),
                "asha".to_string(),
                "catherine".to_string()
            ]
        );
        obj.death("bob".to_string());
        assert_eq!(
            obj.get_inheritance_order(),
            vec![
                "king".to_string(),
                "andy".to_string(),
                "matthew".to_string(),
                "alex".to_string(),
                "asha".to_string(),
                "catherine".to_string()
            ]
        );
    }
}
