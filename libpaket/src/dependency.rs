use semver::Version;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Dependency {
    name: String,
    version: Version,
    dependency_list: Vec<Dependency>,
}

impl Dependency {
    pub fn new(name: impl Into<String>, version: Version) -> Dependency {
        Dependency {
            name: name.into(),
            version,
            dependency_list: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependency_list.push(dependency);
    }

    pub fn get_dependencies(&self) -> &Vec<Dependency> {
        &self.dependency_list
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }

    pub fn list_all_dependencies(&self) -> HashSet<&str> {
        let mut unique_dependency_names = HashSet::new();

        for d in self.get_dependencies() {
            unique_dependency_names.insert(d.get_name().as_str());
            unique_dependency_names.extend(d.list_all_dependencies());
        }

        unique_dependency_names
    }
}
