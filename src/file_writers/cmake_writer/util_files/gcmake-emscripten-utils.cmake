function( apply_emscripten_specifics
  preload_flags_receiver
  actual_target
)
  if( USING_EMSCRIPTEN )
    set( target_file_base "${MY_RUNTIME_OUTPUT_DIR}/${actual_target}" )

    file( GLOB_RECURSE all_resource_file_paths CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/resources/**" )
    list( LENGTH all_resource_file_paths all_resources_count )

    if( all_resources_count GREATER 0 )
      get_target_property( target_type ${preload_flags_receiver} TYPE )

      if( target_type STREQUAL "EXECUTABLE" )
        set( resource_inheritance_mode PRIVATE )
      elseif( target_type STREQUAL "INTERFACE_LIBRARY" )
        set( resource_inheritance_mode INTERFACE )
      else() # Is compiled library
        set( resource_inheritance_mode PUBLIC )
      endif()

      target_link_options( ${preload_flags_receiver} ${resource_inheritance_mode} --preload-file "${CMAKE_CURRENT_SOURCE_DIR}/resources@/resources/" )

      set( hook_file_dir "${CMAKE_BINARY_DIR}/pre-js-hooks" )
      file( MAKE_DIRECTORY "${hook_file_dir}" )

      set( hook_file_name "${hook_file_dir}/${actual_target}.js" )
      
      file( WRITE
        "${hook_file_name}"
        "
          function doLocateFile(path, prefix) {
            if (typeof process !== 'undefined' && process.argv[1]) {
              const modifiedPath = require('path').resolve(
                process.argv[1],
                '..',
                prefix,
                path
              );

              return modifiedPath;
            }
            else {
              return prefix + path;
            }
          }

          // Ensure the module exists. Redeclaration with var in JavaScript
          // is not an error.
          var Module = Module ? Module : {};
          Module['locateFile'] = doLocateFile;
        "
      )

      target_link_options( ${actual_target}
        PRIVATE
          # It's very important that the hook file content is added to the very beginning of the
          # JS output, not just before the user's content runs. The 'locateFile' module hook
          # function must be present when the script is initially setting up in order
          # to correct the .data file loading paths when run by node.
          --extern-pre-js "${hook_file_name}"
      )
    endif()

    set( additional_files_list
      "${target_file_base}.data"
      "${target_file_base}.wasm"
      "${target_file_base}.wasm.map"
    )

    if( EMSCRIPTEN_MODE STREQUAL "Browser" )
      list( APPEND additional_files_list "${target_file_base}.js" )
    endif()

    set_property(
      TARGET ${actual_target}
      APPEND PROPERTY
      ADDITIONAL_CLEAN_FILES ${additional_files_list}
    )
  endif()
endfunction()

macro( configure_emscripten_mode
  default_mode
)
  if( NOT ALREADY_CONFIGURED_EMSCRIPTEN_MODE )
    # Browser
    # NodeJS
    set( EMSCRIPTEN_MODE ${default_mode} CACHE STRING "'Browser' builds an html file and js/wasm runnable in a web browser. 'NodeJS' omits the html file and just creates a js file runnable by NodeJS." )

    set( valid_emscripten_modes "Browser" "NodeJS" )
    set_property( CACHE EMSCRIPTEN_MODE PROPERTY STRINGS ${valid_emscripten_modes} )

    if( EMSCRIPTEN_MODE STREQUAL "Browser" )
      set( CMAKE_EXECUTABLE_SUFFIX ".html" )
    elseif( EMSCRIPTEN_MODE STREQUAL "NodeJS" )
      set( CMAKE_EXECUTABLE_SUFFIX ".js" )
    else()
      message( FATAL_ERROR "Given EMCRIPTEN_MODE '${EMSCRIPTEN_MODE}' is invalid. Must be one of: ${valid_emscripten_modes}" )
    endif()

    message( "Using Emscripten mode: ${EMSCRIPTEN_MODE}" )
    set( ALREADY_CONFIGURED_EMSCRIPTEN_MODE TRUE )
  endif()
endmacro()