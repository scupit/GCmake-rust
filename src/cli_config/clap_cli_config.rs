use clap::{AppSettings, ArgEnum, Subcommand, Args, Parser, ValueEnum};

const SKY: &'static str = "Skylar Cupit";

#[derive(ValueEnum, Clone)]
pub enum CLIProjectOutputTypeIn {
  Exe,
  StaticLib,
  SharedLib,
  HeaderOnly,
  CompiledLib
}

#[derive(Parser)]
#[clap(version = "1.3.3", author = SKY)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
  #[clap(default_value = ".")]
  pub project_root: String,

  #[clap(subcommand)]
  pub subcommand: Option<SubCommandStruct>
}

#[derive(Subcommand)]
pub enum SubCommandStruct {
  /// Subcommand for generating new root projects, subprojects, and tests.
  #[clap(subcommand)]
  New(NewProjectSubcommand),

  // TODO: Change this so that multiple file sets can be created with one command.
  /// Generate code files in-tree.
  #[clap()]
  GenFile(CreateFilesCommand),

  /// Subcommand for working with the 'external dependency configuration repository'.
  #[clap(subcommand)]
  DepConfig(DepConfigSubCommand),

  /// Copy a default file from ~/.gcmake into the project root.
  UseFile(UseFilesCommand),

  /// Select and print information about project outputs and pre-build script.
  TargetInfo(TargetInfoCommand),
  
  /// Select and print information about projects. Dependency print information is limited.
  ProjectInfo(ProjectInfoCommand),

  /// Select and print information about predefined dependencies
  PredepInfo(PredepInfoCommand)
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum NewProjectSubcommand {
  /// Generate a new toplevel project
  #[clap()]
  RootProject(NewProjectCommand),

  /// Generate a new subproject
  #[clap()]
  Subproject(NewSubprojectCommand),

  /// Generate a new test
  #[clap()]
  Test(NewTestProjectCommand)
}

/// Generate a new toplevel project
#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct NewProjectCommand {
  /// Project name, no whitespace
  #[clap(required = true)]
  pub new_project_name: String,

  /// Generate a C project and skip language prompt.
  #[clap(long)]
  pub c: bool,

  /// Generate a C++ project and skip language prompt.
  #[clap(long)]
  pub cpp: bool,

  #[clap(value_enum, short, long, name = "type")]
  pub project_type: Option<CLIProjectOutputTypeIn>,
}

/// Generate a new subproject
#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct NewSubprojectCommand {
  /// Project name, no whitespace
  #[clap(required = true)]
  pub new_project_name: String,

  /// Generate a C project and skip language prompt.
  #[clap(long)]
  pub c: bool,

  /// Generate a C++ project and skip language prompt.
  #[clap(long)]
  pub cpp: bool,

  #[clap(value_enum, short, long, name = "type")]
  pub subproject_type: Option<CLIProjectOutputTypeIn>
}

/// Generate a new test.
/// Note that all tests are C++ executable subprojects, since only C++
/// test frameworks are currently supported.
#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct NewTestProjectCommand {
  /// Project name, no whitespace
  #[clap(required = true)]
  pub new_project_name: String,
}

#[derive(ArgEnum, Clone, Copy)]
pub enum FileCreationLang {
  C,
  Cpp
}

#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CreateFilesCommand {
  #[clap(arg_enum, required = true)]
  pub language: FileCreationLang,

  /// Combination of 'h' 's' and 't' (example: hs), where 'h' = Header, 's' = Source,
  /// and 't' = Template implementation
  #[clap(long = "which", default_value = "hs")]
  pub which: String,

  /// Name of the generated file relative to any code folder.
  /// Example: Assuming file_types == hs (header and source generated)
  /// and language == cpp,
  /// "SomeClass" turns into "include/<FULL_INCLUDE_PREFIX>/SomeClass.hpp" and "src/<FULL_INCLUDE_PREFIX>/SomeClass.cpp" 
  /// while "nested/SomeClass" turns into "include/<FULL_INCLUDE_PREFIX>/nested/SomeClass.hpp" and
  /// "src/<FULL_INCLUDE_PREFIX>/nested/SomeClass.cpp" 
  #[clap(required = true)]
  pub relative_file_names: Vec<String>,

  /// Use '#pragma once' instead of include guards.
  #[clap(short = 'p', long = "use-pragma")]
  pub use_pragma_guards: bool
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::ColoredHelp)]
pub enum DepConfigSubCommand {
  /// Update the dependency configuration repo. Downloads the repo if it is not already present.
  #[clap()]
  Update(UpdateDependencyConfigsCommand)
}

#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct UpdateDependencyConfigsCommand {
  /// Selects the branch to be checked out before pulling changes (or after cloning, if the
  /// repo hasn't been installed yet). If no branch is specified, then the current branch is
  /// updated or the repo is cloned into the 'develop' branch.
  #[clap(long = "to-branch", short = 'b')]
  pub branch: Option<String>,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum UseFileOption {
  #[clap(name = "clang-tidy")]
  ClangTidy,

  #[clap(name = "clang-format")]
  ClangFormat,

  #[clap(name = "gitignore")]
  GitIgnore
}

impl UseFileOption {
  pub fn to_file_name(&self) -> &str {
    match self {
      Self::ClangTidy => ".clang-tidy",
      Self::ClangFormat => ".clang-format",
      Self::GitIgnore => ".gitignore"
    }
  }
}

#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct UseFilesCommand {
  /// The file to copy, without the leading '.'
  #[clap(value_enum)]
  pub file: UseFileOption
}

#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TargetInfoCommand {
  /// Select which targets to print info for. Can be in namespace format 'self::the-target'
  /// 'some-project::{ first-target, second-target }', or just a lone target name
  /// 'the-target'. Lone target names only select from targets in the project tree,
  /// but namespaces are able to select dependency targets as well.
  #[clap(required = true)]
  pub selectors: Vec<String>,

  /// Print the include path of the auto-generated export header
  #[clap(short = 'e')]
  pub export_header: bool
}

#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectInfoCommand {
  /// Select which projects to print info for 
  #[clap(required = false)]
  pub selectors: Vec<String>,

  /// Print the project's full include prefix
  #[clap(short = 'i')]
  pub show_include_prefix: bool,

  /// List immediate subprojects
  #[clap(short = 's')]
  pub show_subprojects: bool,
}

#[derive(Args)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct PredepInfoCommand {
  /// Select which predefined dependencies to print info for. If no selectors are provided,
  /// then the full list of predefined dependencies is printed out.
  #[clap(required = false)]
  pub selectors: Vec<String>,

  /// List out all the dependency's available targets
  #[clap(short = 't')]
  pub show_targets: bool
}