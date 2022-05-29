use std::collections::HashMap;

use serde::{Serialize, Deserialize};

pub enum RawDep<'a> {
  AsSubdirectory(&'a RawSubdirectoryDependency)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SinglePredefinedDependencyInfo {
  as_subdirectory: RawSubdirectoryDependency
}

// Container for all dependency types defined in supported_dependencies.yaml
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllPredefinedDependencies {
  as_subdirectory: HashMap<String, RawSubdirectoryDependency>
  // prewritten_find_modules:
  // non_subdirectory_cmake_projects:
}

impl AllPredefinedDependencies {
  pub fn new() -> AllPredefinedDependencies {
    AllPredefinedDependencies {
      as_subdirectory: HashMap::new()
    }
  }

  pub fn add(
    &mut self,
    dep_name: impl Into<String>,
    dep_info: SinglePredefinedDependencyInfo
  ) {
    self.as_subdirectory.insert(dep_name.into(), dep_info.as_subdirectory);
  }

  pub fn find_dependency(&self, dep_name: &str) -> Option<RawDep> {
    if let Some(subdir_dep) = self.as_subdirectory.get(dep_name) {
      return Some(RawDep::AsSubdirectory(subdir_dep))
    }

    None
  }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamespaceConfig {
  used_in_cmake_yaml: String,
  cmakelists_linking: String
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitRepoConfig {
  pub repo_url: String
}

// A predefined dependency which exists within the project build tree.
// These should always be inside the dep/ folder.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSubdirectoryDependency {
  namespace_config: NamespaceConfig,
  pub git_repo: GitRepoConfig,
  pub target_names: Vec<String>
}

impl RawSubdirectoryDependency {
  pub fn namespaced_target(&self, target_name: &str) -> Option<String> {
    for raw_target_name in &self.target_names {
      if raw_target_name == target_name {
        return Some(format!("{}{}", self.namespace_config.cmakelists_linking, target_name))
      }
    }
    None
  }
}