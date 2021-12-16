use std::{collections::HashMap, fs::{self}, io, iter::FromIterator, path::{PathBuf}};


pub struct CMakeUtilWriter {
  cmake_utils_path: PathBuf,
  utils: HashMap<&'static str, &'static str>
}

impl CMakeUtilWriter {
  pub fn new(cmake_utils_path: PathBuf) -> Self {
    return Self {
      cmake_utils_path,
      utils: HashMap::from_iter([
        ("toggle-lib-util", TOGGLE_LIB_UTIL_TEXT),
        ("exe-prebuild-util", EXE_PREBUILD_UTIL_TEXT),
        ("python-prebuild-util", PYTHON_PREBUILD_UTIL_TEXT)
      ])
    }
  }

  pub fn write_cmake_utils(&self) -> io::Result<()> {
    if !self.cmake_utils_path.is_dir() {
      fs::create_dir(&self.cmake_utils_path)?;
    }

    for (util_name, util_contents) in &self.utils {
      let mut util_file_path = self.cmake_utils_path.join(util_name);
      util_file_path.set_extension("cmake");

      fs::write(
        util_file_path,
        util_contents
      )?;
    }

    Ok(())
  }

  pub fn get_utils(&self) -> &HashMap<&'static str, &'static str> {
    &self.utils
  }
}


const TOGGLE_LIB_UTIL_TEXT: &'static str = 
r#"function( make_toggle_lib
  lib_name
  default_lib_type
  lib_entry_file
  lib_sources
  lib_headers
  lib_template_impls
)
  if (NOT "${default_lib_type}" STREQUAL "STATIC" AND NOT "${default_lib_type}" STREQUAL "SHARED")
    message( FATAL_ERROR "Invalid default lib type '${default_lib_type}' given to type toggleable library ${lib_name}" )
  endif()

  if( NOT ${lib_name}_LIB_TYPE )
    set( ${lib_name}_LIB_TYPE ${default_lib_type} CACHE STRING "Library type for ${lib_name}" )
  endif()

  set_property( CACHE ${lib_name}_LIB_TYPE PROPERTY STRINGS "STATIC" "SHARED" )

  set( all_lib_files ${lib_entry_file} ${lib_sources} ${lib_headers} ${lib_template_impls} )

  if ( ${lib_name}_LIB_TYPE STREQUAL STATIC )
    add_library( ${lib_name} STATIC ${all_lib_files})
  elseif( ${lib_name}_LIB_TYPE STREQUAL SHARED )
    add_library( ${lib_name} SHARED ${all_lib_files})
  endif()
endfunction()
"#;

const EXE_PREBUILD_UTIL_TEXT: &'static str = 
r#"function( use_executable_prebuild_script
  target_name
)
  add_custom_target( ${PROJECT_NAME}-pre-build-step
    ALL
    COMMAND ${target_name}
    DEPENDS ${target_name}
    COMMENT "Running ${PROJECT_NAME} pre-build executable script"
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  )
endfunction()
"#;

const PYTHON_PREBUILD_UTIL_TEXT: &'static str = 
r#"function( use_python_prebuild_script
  python_prebuild_file
)
  include( FindPython3 )
  find_package( Python3 COMPONENTS Interpreter )

  if( ${Python3_FOUND} AND ${Python3_Interpreter_FOUND} )
    add_custom_target( ${PROJECT_NAME}-pre-build-step
      ALL
      COMMAND Python3::Interpreter ${python_prebuild_file}
      COMMENT "Running ${PROJECT_NAME} pre-build python script"
      WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    )
  else()
    if( NOT ${Python3_Interpreter_FOUND} )
      message( FATAL_ERROR "A Python 3 interpreter is needed to run the pre-build script for project ${PROJECT_NAME}, however a valid interpreter was not found." )
    else()
      message( FATAL_ERROR "Unable to find a valid Python 3 configuration when configuring project ${PROJECT_NAME}" )
    endif()
  endif()
endfunction()
"#;