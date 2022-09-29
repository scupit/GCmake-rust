function( exe_add_lib_relative_install_rpath
  exe_target
)
  if( NOT TARGET_SYSTEM_IS_WINDOWS )
    set_property(
      TARGET ${exe_target}
      APPEND PROPERTY
        INSTALL_RPATH "\${ORIGIN}/../lib"
    )
  endif()
endfunction()

function( shared_lib_add_relative_install_rpath
  shared_lib_target
)
  if( NOT TARGET_SYSTEM_IS_WINDOWS )
    set_property(
      TARGET ${shared_lib_target}
      APPEND PROPERTY
        INSTALL_RPATH "\${ORIGIN}"
    )
  endif()
endfunction()

function( clean_list
  content
  output_var
)
  string( REGEX REPLACE "(^ *;)|(; *$)" "" cleaned_list_out "${content}" )
  # string( REGEX REPLACE ";" " " cleaned_list_out "${cleaned_list_out}" )
  set( ${output_var} "${cleaned_list_out}" PARENT_SCOPE )
endfunction()

# TODO: Refactor these two into one delegator function
function( get_without_toplevel_dir_prefix
  all_files
  receiving_var
)
  string( REPLACE "${TOPLEVEL_PROJECT_DIR}/" "" with_removed_prefix "${all_files}" )
  string( REPLACE "./" "" with_removed_prefix "${with_removed_prefix}" )
  clean_list( "${with_removed_prefix}" with_removed_prefix )
  set( ${receiving_var} "${with_removed_prefix}" PARENT_SCOPE )
endfunction()

function( get_without_source_dir_prefix
  all_files
  receiving_var
)
  string( REPLACE "${CMAKE_CURRENT_SOURCE_DIR}/" "" with_removed_prefix "${all_files}" )
  string( REPLACE "./" "" with_removed_prefix "${with_removed_prefix}" )
  clean_list( "${with_removed_prefix}" with_removed_prefix )
  set( ${receiving_var} "${with_removed_prefix}" PARENT_SCOPE )
endfunction()

function( make_generators
  for_build
  for_install
  var_name
)
  foreach( file_for_build IN LISTS for_build )
    set( ${var_name}_b "${${var_name}_b}" "$<BUILD_INTERFACE:${file_for_build}>" )
  endforeach()

  foreach( file_for_install IN LISTS for_install )
    set( ${var_name}_i "${${var_name}_i}" "$<INSTALL_INTERFACE:${file_for_install}>" )
  endforeach()

  set( ${var_name}_b "${${var_name}_b}" PARENT_SCOPE )
  set( ${var_name}_i "${${var_name}_i}" PARENT_SCOPE )
endfunction()

function( apply_exe_files
  exe_target
  receiver_target
  entry_file
  sources
  headers
  template_impls
)
  set( receiver_interface_lib ${receiver_target} )

  clean_list( "${entry_file}" entry_source )
  get_without_source_dir_prefix( "${entry_source}" entry_source_install_interface )
  make_generators( "${entry_source}" "${entry_source_install_interface}" entry_source_gens )
  target_sources( ${exe_target} PRIVATE
    ${entry_source_gens_b}
    ${entry_source_gens_i}
  )

  # set( non_entry_sources ${entry_file};${sources} )
  set( non_entry_sources ${sources} )
  clean_list( "${non_entry_sources}" non_entry_sources )
  get_without_source_dir_prefix( "${non_entry_sources}" all_sources_install_interface )

  make_generators( "${non_entry_sources}" "${all_sources_install_interface}" source_gens )
  target_sources( ${receiver_interface_lib} INTERFACE
    ${source_gens_b}
    ${source_gens_i}
  )

  set( all_headers "${headers};${template_impls}" )
  clean_list( "${all_headers}" all_headers )

  if( NOT "${all_headers}" STREQUAL "" )
    get_without_source_dir_prefix( "${all_headers}" all_headers_install_interface )

    make_generators( "${all_headers}" "${all_headers_install_interface}" header_gens )
    target_sources( ${receiver_interface_lib} INTERFACE
      FILE_SET HEADERS
        FILES
          ${header_gens_b}
          ${header_gens_i}
    )
  endif()
endfunction()

function( get_entry_file_alias_dir
  out_var
)
  set( ${out_var} "${CMAKE_BINARY_DIR}/aliased_entry_files/include" PARENT_SCOPE )
endfunction()

function( apply_lib_files
  lib_target
  lib_type_spec
  entry_file
  sources
  headers
  template_impls
)
  if( NOT "${lib_type_spec}" STREQUAL "COMPILED_LIB" AND NOT "${lib_type_spec}" STREQUAL "HEADER_ONLY_LIB" )
    message( FATAL_ERROR "Invalid lib type spec '${lib_type_spec}' given to apply_lib_files(...)" )
  endif()

  if( "${lib_type_spec}" STREQUAL "COMPILED_LIB" )
    clean_list( "${sources}" non_entry_sources)

    if( NOT "${non_entry_sources}" STREQUAL "" )
      get_without_source_dir_prefix( "${non_entry_sources}" all_sources_install_interface )

      make_generators( "${non_entry_sources}" "${all_sources_install_interface}" source_gens )
      target_sources( ${lib_target}
        PRIVATE
          ${source_gens_b}
          ${source_gens_i}
      )
    endif()
  endif()

  cmake_path( GET entry_file FILENAME entry_file_name )

  # Want to make sure entry files can be included with "TOPLEVEL_INCLUDE_PREFIX/entry_file_name.extension"
  # Both when building and after installation in order to eliminate possible include issues.
  get_entry_file_alias_dir( entry_file_alias_dir )
  set( aliased_entry_file_path "${entry_file_alias_dir}/${TOPLEVEL_INCLUDE_PREFIX}/${entry_file_name}" )
  file( MAKE_DIRECTORY "${entry_file_alias_dir}/${TOPLEVEL_INCLUDE_PREFIX}" )
  file( CREATE_LINK "${entry_file}" "${aliased_entry_file_path}" COPY_ON_ERROR )

  set( all_headers "${entry_file};${headers};${template_impls}" )
  clean_list( "${all_headers}" all_headers )

  get_without_source_dir_prefix( "${all_headers}" all_headers_install_interface )

  if( "${lib_type_spec}" STREQUAL "HEADER_ONLY_LIB" )
    set( header_inheritance_mode INTERFACE )
  else()
    set( header_inheritance_mode PUBLIC )
  endif()

  make_generators( "${all_headers}" "${all_headers_install_interface}" header_gens )
  target_sources( ${lib_target} ${header_inheritance_mode}
    FILE_SET HEADERS
      FILES
        ${header_gens_b}
        ${header_gens_i}
  )
endfunction()

function( apply_include_dirs
  target
  target_type
  project_include_dir
)
  if( "${target_type}" STREQUAL "COMPILED_LIB" OR "${target_type}" STREQUAL "HEADER_ONLY_LIB" )
    get_entry_file_alias_dir( entry_file_alias_dir )
    set( BUILD_INTERFACE_INCLUDE_DIRS "${entry_file_alias_dir};${project_include_dir}")
  elseif( "${target_type}" STREQUAL "EXE_RECEIVER" OR "${target_type}")
    set( BUILD_INTERFACE_INCLUDE_DIRS "${project_include_dir}")
  else()
    message( FATAL_ERROR "Invalid target_type '${target_type}' given to function 'apply_include_dirs'" )
  endif()

  if( "${target_type}" STREQUAL "HEADER_ONLY_LIB" )
    set( include_dir_inheritance_mode INTERFACE )
  elseif( "${target_type}" STREQUAL "EXE_RECEIVER" )
    set( include_dir_inheritance_mode INTERFACE )
  else()
    set( include_dir_inheritance_mode PUBLIC )
  endif()

  target_include_directories( ${target}
    ${include_dir_inheritance_mode}
      "$<BUILD_INTERFACE:${BUILD_INTERFACE_INCLUDE_DIRS}>"
      "$<INSTALL_INTERFACE:include>"
      "$<INSTALL_INTERFACE:include/${TOPLEVEL_INCLUDE_PREFIX}/include>"
  )
endfunction()

function( initialize_build_tests_var )
  set( option_description "Whether to build tests for the ${PROJECT_NAME} project tree." )

  if( "${CMAKE_SOURCE_DIR}" STREQUAL "${CMAKE_CURRENT_SOURCE_DIR}" AND NOT CMAKE_CROSSCOMPILING )
    option( ${LOCAL_TOPLEVEL_PROJECT_NAME}_BUILD_TESTS "${option_description}" ON )
  else()
    option( ${LOCAL_TOPLEVEL_PROJECT_NAME}_BUILD_TESTS "${option_description}" OFF )
  endif()
endfunction()

function( initialize_build_config_vars )
  set( ALL_CONFIGS_LOCAL_DEFINES )

  foreach( config_name IN ITEMS "Debug" "Release" "MinSizeRel" "RelWithDebInfo" )
    set( ${config_name}_LOCAL_COMPILER_FLAGS )
    set( ${config_name}_LOCAL_LINKER_FLAGS )
    set( ${config_name}_LOCAL_DEFINES )
  endforeach()
endfunction()

function( propagate_all_configs_local_defines )
  foreach( config_name IN ITEMS "Debug" "Release" "MinSizeRel" "RelWithDebInfo" )
    list( APPEND ${config_name}_LOCAL_DEFINES ${ALL_CONFIGS_LOCAL_DEFINES} )
  endforeach()
endfunction()

macro( initialize_ipo_defaults
  ipo_on_by_default
)
  if( NOT IPO_DEFAULTS_INITIALIZED )
    include( CheckIPOSupported )

    check_ipo_supported(
      RESULT is_ipo_supported
      OUTPUT additional_info
    )

    if( is_ipo_supported )
      if( USING_MINGW )
        set( IPO_ENABLED_DEFAULT_VALUE OFF )
      else()
        set( IPO_ENABLED_DEFAULT_VALUE ${ipo_on_by_default} )
      endif()

      option(
        GCMAKE_ENABLE_IPO
        "When set to ON, enables INTERPROCEDURAL_OPTIMIZATION for the whole project tree (including dependencies built as part of the project)"
        ${IPO_ENABLED_DEFAULT_VALUE}
      )

      set( CMAKE_INTERPROCEDURAL_OPTIMIZATION ${GCMAKE_ENABLE_IPO} )
      message( STATUS "Interprocedural Optimization: ${CMAKE_INTERPROCEDURAL_OPTIMIZATION}" )
    else()
      message( WARNING "Skipping enabling IPO because is isn't supported. Additional info: ${additional_info}" )
    endif()

    set( IPO_DEFAULTS_INITIALIZED TRUE )
  endif()
endmacro()

function( initialize_lib_type_options
  DEFAULT_COMPILED_LIB_TYPE
)
  if( "${DEFAULT_COMPILED_LIB_TYPE}" STREQUAL "STATIC" )
    set( SHOULD_DEFAULT_TO_STATIC ON )
    set( SHOULD_DEFAULT_TO_SHARED OFF )
  elseif( "${DEFAULT_COMPILED_LIB_TYPE}" STREQUAL "SHARED" )
    set( SHOULD_DEFAULT_TO_STATIC OFF )
    set( SHOULD_DEFAULT_TO_SHARED ON )
  else()
    message( FATAL_ERROR "(GCMake error): DEFAULT_COMPILED_LIB_TYPE should be set to either STATIC or SHARED, but is set to invalid value '${DEFAULT_COMPILED_LIB_TYPE}'.")
  endif()

  option( BUILD_SHARED_LIBS "${LOCAL_BUILD_SHARED_LIBS_DOC_STRING}" ${SHOULD_DEFAULT_TO_SHARED} )
  option( BUILD_STATIC_LIBS "${LOCAL_BUILD_STATIC_LIBS_DOC_STRING}" ${SHOULD_DEFAULT_TO_STATIC} )
endfunction()

function( unaliased_target_name
  target_name
  out_var
)
  get_target_property( unaliased_lib_name ${target_name} ALIASED_TARGET )

  if( NOT unaliased_lib_name )
    set( unaliased_lib_name ${target_name} )
  endif()

  set( ${out_var} ${unaliased_lib_name} PARENT_SCOPE )
endfunction()