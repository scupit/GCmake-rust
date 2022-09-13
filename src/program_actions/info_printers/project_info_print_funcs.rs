use crate::project_info::{dependency_graph_mod::dependency_graph::{DependencyGraph, ProjectWrapper}, final_dependencies::FinalPredepInfo};
use colored::*;

pub fn print_project_header(project: &DependencyGraph) {
  println!("\n========== {} ==========", project.project_debug_name().green());
}

pub fn print_project_include_prefix(project_graph: &DependencyGraph) {
  match project_graph.project_wrapper().maybe_normal_project() {
    Some(normal_project) => {
      println!("Include prefix: {}", normal_project.get_full_include_prefix())
    },
    None => println!("Cannot determine include prefix.")
  }
}

pub fn print_immediate_subprojects(project_graph: &DependencyGraph) {
  match project_graph.project_wrapper().maybe_normal_project() {
    Some(_) => {
      print!("Subprojects: ");
      
      if project_graph.get_subprojects().is_empty() {
        println!("None");
      }
      else {
        println!();
        for (subproject_name, _) in project_graph.get_subprojects() {
          println!("\t- {}", subproject_name);
        }
      }
    },
    None => println!("Cannot determine subprojects")
  }
}

fn extract_repo_url(project_graph: &DependencyGraph) -> Result<String, String> {
  return match project_graph.project_wrapper() {
    ProjectWrapper::PredefinedDependency(predef_def) => match predef_def.predefined_dep_info() {
      FinalPredepInfo::Subdirectory(subdir_dep) => Ok(subdir_dep.repo_url().to_string()),
      _ => {
        Err(format!(
          "Repository information is not available for \"{}\" because it is retrieved from the system, not cloned as part of the build.",
          project_graph.project_debug_name()
        ))
      }
    },
    ProjectWrapper::GCMakeDependencyRoot(gcmake_dep) => Ok(gcmake_dep.repo_url().to_string()),
    // Normal projects don't have a repository URL because they are part of the repository itself. However,
    // the root of a normal project may be a GCMake dependency.
    ProjectWrapper::NormalProject(_) => {
      if let ProjectWrapper::GCMakeDependencyRoot(root) = project_graph.root_project().as_ref().borrow().project_wrapper() {
        Ok(root.repo_url().to_string())
      }
      else {
        Err(format!("No repo URL"))
      }
    }
  }
  
}

pub fn print_project_repo_url(project_graph: &DependencyGraph) {
  match extract_repo_url(project_graph) {
    Ok(url) => println!("Repo URL:\n\t{}", url),
    Err(reason_missing) => println!("{}", reason_missing)
  }
}
