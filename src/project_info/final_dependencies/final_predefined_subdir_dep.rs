use std::{collections::{HashMap, HashSet}};

use crate::project_info::{raw_data_in::dependencies::{internal_dep_config::{RawSubdirectoryDependency, raw_dep_common::RawPredepCommon}, user_given_dep_config::{UserGivenPredefinedDependencyConfig}}};

use super::{predep_module_common::PredefinedDepFunctionality, final_target_map_common::{FinalTargetConfigMap, make_final_target_config_map}};

#[derive(Clone)]
pub struct FinalSubdirDepLinks {
  github: Option<String>
}

impl FinalSubdirDepLinks {
  pub fn new_empty() -> Self {
    Self {
      github: None
    }
  }
}

#[derive(Clone)]
pub enum GitRevisionSpecifier {
  Tag(String),
  CommitHash(String)
}

// Unused for now, but will be required when using config-mode find_package for
// CMake dependencies already installed on the system.
pub enum DependencyVersion {
  MinRequired(String),
  Exact(String)
}

#[derive(Clone)]
pub struct FinalGitRepoDescriptor {
  pub repo_url: String,
  pub revision_specifier: GitRevisionSpecifier
}

#[derive(Clone)]
pub struct SubdirDepInstallationConfig {
  pub var_name: String,
  pub is_inverse: bool,
  pub should_install_by_default: bool
}

#[derive(Clone)]
pub struct PredefinedSubdirDep {
  git_repo: FinalGitRepoDescriptor,
  installed_include_dir_name: Option<String>,
  // Unused for now, but may be used in the future to propagate installed DLLs from the gcmake project
  // install dir on Windows.
  // TODO: Do that, actually.
  config_file_project_name: Option<String>,
  // Map of target base name to the namespaced target name used for linking.
  target_map: FinalTargetConfigMap,
  cmake_namespaced_target_map: HashMap<String, String>,
  yaml_namespaced_target_map: HashMap<String, String>,
  requires_custom_populate: bool,
  installation_details: Option<SubdirDepInstallationConfig>,
  url_links: FinalSubdirDepLinks,
  _can_cross_compile: bool
}

impl PredefinedSubdirDep {
  pub fn get_target_config_map(&self) -> &FinalTargetConfigMap {
    &self.target_map
  }

  pub fn get_installation_details(&self) -> &Option<SubdirDepInstallationConfig> {
    &self.installation_details
  }

  pub fn custom_relative_include_dir_name(&self) -> &Option<String> {
    &self.installed_include_dir_name
  }

  pub fn different_config_file_project_name(&self) -> &Option<String> {
    &self.config_file_project_name
  }

  pub fn requires_custom_fetchcontent_populate(&self) -> bool {
    self.requires_custom_populate
  }

  pub fn get_cmake_linkable_target_name(&self, target_name: &str) -> Option<&str> {
    self.cmake_namespaced_target_map.get(target_name)
      .map(|str_ref| &str_ref[..])
  }

  pub fn get_yaml_linkable_target_name(&self, target_name: &str) -> Option<&str> {
    self.yaml_namespaced_target_map.get(target_name)
      .map(|str_ref| &str_ref[..])
  }

  pub fn has_target_named(&self, target_name: &str) -> bool {
    self.target_map.get(target_name).is_some()
  }

  pub fn repo_url(&self) -> &str {
    &self.git_repo.repo_url
  }

  pub fn get_url_links(&self) -> &FinalSubdirDepLinks {
    &self.url_links
  }

  pub fn revision(&self) -> &GitRevisionSpecifier {
    &self.git_repo.revision_specifier
  }

  pub fn from_subdir_dep(
    subdir_dep: &RawSubdirectoryDependency,
    user_given_config: &UserGivenPredefinedDependencyConfig,
    dep_name: &str
  ) -> Result<Self, String> {
    let revision_specifier: GitRevisionSpecifier = if let Some(tag_string) = &user_given_config.git_tag {
      GitRevisionSpecifier::Tag(tag_string.clone())
    }
    else if let Some(hash_string) = &user_given_config.commit_hash {
      GitRevisionSpecifier::CommitHash(hash_string.clone())
    }
    else {
      return Err(format!("Must specify either a commit_hash or git_tag for dependency '{}'", dep_name));
    };

    let target_map = make_final_target_config_map(dep_name, subdir_dep)
      .map_err(|err_msg| format!(
        "When loading predefined subdirectory dependency \"{}\":\n\n{}",
        dep_name,
        err_msg
      ))?;

    let mut cmake_namespaced_target_map: HashMap<String, String> = HashMap::new();

    for (target_name, target_config) in &target_map {
      cmake_namespaced_target_map.insert(
        target_name.to_string(),
        format!(
          "{}{}",
          &subdir_dep.namespace_config.cmakelists_linking,
          &target_config.cmakelists_name
        )
      );
    }

    let mut yaml_namespaced_target_map: HashMap<String, String> = HashMap::new();

    for (target_name, target_config) in &target_map {
      yaml_namespaced_target_map.insert(
        target_name.to_string(),
        format!(
          "{}::{}",
          dep_name.to_string(),
          &target_config.cmake_yaml_name
        )
      );
    }

    let should_install_by_default: bool = subdir_dep.install_by_default.unwrap_or(true);

    let installation_details: Option<SubdirDepInstallationConfig> = match (&subdir_dep.install_var, &subdir_dep.inverse_install_var) {
      (Some(install_var), _) => {
        Some(SubdirDepInstallationConfig {
          var_name: install_var.clone(),
          is_inverse: false,
          should_install_by_default
        })
      },
      (_, Some(inverse_install_var)) => {
        Some(SubdirDepInstallationConfig {
          var_name: inverse_install_var.clone(),
          is_inverse: true,
          should_install_by_default
        })
      },
      _ => None
    };

    let links_config: FinalSubdirDepLinks = match &subdir_dep.links {
      None => FinalSubdirDepLinks::new_empty(),
      Some(raw_links_config) => {
        FinalSubdirDepLinks {
          github: raw_links_config.github.clone()
        }
      }
    };

    return Ok(
      Self {
        url_links: links_config,
        git_repo: FinalGitRepoDescriptor {
          revision_specifier,
          repo_url: user_given_config.repo_url.clone()
            .unwrap_or(subdir_dep.git_repo.repo_url.clone())
        },
        installed_include_dir_name: subdir_dep.installed_include_dir_name.clone(),
        config_file_project_name: subdir_dep.config_file_project_name.clone(),
        target_map,
        cmake_namespaced_target_map,
        yaml_namespaced_target_map,
        requires_custom_populate: subdir_dep.requires_custom_fetchcontent_populate,
        installation_details,
        _can_cross_compile: subdir_dep.can_trivially_cross_compile()
      }
    )
  }
}

impl PredefinedDepFunctionality for PredefinedSubdirDep {
  fn can_cross_compile(&self) -> bool {
    self._can_cross_compile
  }

  fn get_target_config_map(&self) -> &FinalTargetConfigMap {
    &self.target_map
  }

  fn target_name_set(&self) -> HashSet<String> {
    self.target_map.keys()
      .map(|k| k.to_string())
      .collect()
  }
}