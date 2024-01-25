#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::{collections::HashSet, path::Path};

    use libpaket::dependency::*;

    #[test]
    fn empty_dependency() -> Result<(), Box<dyn std::error::Error>> {
        let mut dep_root = Dependency::new("test", "1.0.0".parse()?);

        let dependency_list = dep_root.list_all_dependencies();

        assert_eq!(HashSet::new(), dependency_list);

        Ok(())
    }

    /// Dependency Tree test:
    /// ```
    /// A_1.0.0
    /// ├── B_1.0.0
    /// │   ├── B1SUB_1.0.0
    /// │   └── B2SUB_1.0.0
    /// └── C_1.0.0
    /// ```
    #[test]
    fn line_dependency() -> Result<(), Box<dyn std::error::Error>> {
        let mut dep_root = Dependency::new("A", "1.0.0".parse()?);

        let mut dep_b = Dependency::new("B", "1.0.0".parse()?);
        dep_b.add_dependency(Dependency::new("B1SUB", "1.0.0".parse()?));
        dep_b.add_dependency(Dependency::new("B2SUB", "1.0.0".parse()?));
        dep_root.add_dependency(dep_b);

        dep_root.add_dependency(Dependency::new("C", "1.0.0".parse()?));

        // Get Dependency List
        let dependency_list = dep_root.list_all_dependencies();
        let expected_list: HashSet<&str> = vec!["B", "C", "B1SUB", "B2SUB"].into_iter().collect();

        assert_eq!(expected_list, dependency_list);

        Ok(())
    }

    /// Dependency Tree test:
    /// ```
    /// A.0.0
    /// ├── Node_1.0.0
    /// │   └── B_1.2.0
    /// │       ├── B1SUB_1.1.0
    /// │       └── B2SUB_1.3.0
    /// ├── B_1.0.0
    /// │   ├── B1SUB_1.0.0
    /// │   └── B2SUB_1.0.0
    /// └── C_1.0.0
    /// ```
    #[test]
    fn bigger_dependency_in_node() -> Result<(), Box<dyn std::error::Error>> {
        let mut dep_root = Dependency::new("A", "1.0.0".parse()?);
        let mut dep_node = Dependency::new("Node", "1.0.0".parse()?);

        let mut dep_b = Dependency::new("B", "1.0.0".parse()?);
        dep_b.add_dependency(Dependency::new("B1SUB", "1.0.0".parse()?));
        dep_b.add_dependency(Dependency::new("B2SUB", "1.0.0".parse()?));
        dep_root.add_dependency(dep_b);

        let mut dep_b2 = Dependency::new("B", "1.2.0".parse()?);
        dep_b2.add_dependency(Dependency::new("B1SUB", "1.1.0".parse()?));
        dep_b2.add_dependency(Dependency::new("B2SUB", "1.3.0".parse()?));
        dep_node.add_dependency(dep_b2);

        dep_root.add_dependency(dep_node);

        dep_root.add_dependency(Dependency::new("C", "1.0.0".parse()?));

        // Get Dependency List
        let dependency_list = dep_root.list_all_dependencies();
        let expected_list: HashSet<&str> = vec!["Node", "B", "C", "B1SUB", "B2SUB"]
            .into_iter()
            .collect();

        assert_eq!(expected_list, dependency_list);

        Ok(())
    }
}
