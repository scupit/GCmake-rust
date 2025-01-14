# TODO

There are a whole bunch of things which need doing. This is the place to list them.

## Priorities

- [x] Warn on dependencies which are never used, since that's almost always a mistake.
- [x] Support for parallelism systems ([CUDA](https://developer.nvidia.com/cuda-toolkit), [Kokkos](https://github.com/kokkos/kokkos), and [compiler-builtin OpenMP](https://www.openmp.org/))
  - Don't support HIP because it doesn't support Windows. OpenSYCL barely has experimental Windows support, but its build process looks like a pain in general. OpenCL seems extremely portable, but I couldn't get it to work in a basic CMake project so I won't support it yet.
- [x] Warn when files exist inside *include/* or *src/*, but aren't inside the dir with the include prefix
  (like *include/MY_INCLUDE_PREXIX* or *src/MY_INCLUDE_PREFIX*).
- [ ] Now that predefined dependency configurations are lazy-loaded, add a command for checking the correctness of any given config (or all configs at once). "Correctness checking" just means the config can be loaded without errors.
- [ ] Add CLI commands for cleaning and updating the dep-cache. Not exactly sure how updating should work yet.
- [ ] Have documentation generation depend on all handwritten .rst and .h(pp) files in *docs/*, that way
  CMake knows to rebuild documentation when they change. This is already the case for index.rst, but should
  probably also include the other rst files.
- [ ] "Package manifest" generator for GCMake projects. It's a bit of a pain to use other GCMake projects
  as dependencies because their information is never "forward declared" anywhere, unlike predefined dependencies.
  As a result, we can only generate a proper CMakeLists.txt and fully analyze the project tree once all GCMake
  projects have already been downloaded (by running a CMake configuration). This is not ideal. The end goal
  is to have a package registry for GCMake projects so that the full analysis and generation steps can be
  done in one pass without the dependency projects already being present. This would be a good first
  step towards that.
- [ ] Rename `predefined_dependencies` to something more intuitive. These dependencies are not gcmake
  projects, but can be configured to work with gcmake by providing a 'yaml dependency glue' config.

## Configuration TODO

### General

- [ ] Allow `(( lang:c and lang:cpp ))` constraints for flags and defines courtesy of CMake's
  [$<COMPILE_LANGAUGE>](https://cmake.org/cmake/help/latest/manual/cmake-generator-expressions.7.html#genex:COMPILE_LANGUAGE)
  generator expression. That might not be possible with Visual Studio though (see notes on the linked page).
- [ ] Research and add some more useful default flags (for example, `-Wold-style-cast` for GCC/Clang).
- [ ] Now that minimal installs are implemented, add ability to specify exactly which executables are installed.

#### Explicit Compiler Support

- [ ] Intel C/C++ compiler

### CLI TODO

#### dep-graph

The command set for viewing dependency graph info.

- [ ] `dep-graph` command which prints a dependency graph for each target in the current project.
- [ ] `dep-graph <target>` command which prints a dependency graph for the given target.

#### show

The command set for viewing project metadata.

- [ ] `show linkable` shows available targets per subproject and dependency for the current project.
        Allow a `--from <project-or-dep-name>` flag to specify that only targets/libraries from
        the given subproject/dependency should be printed.
- [ ] `show defines <config-name>` prints the defines specified by the buildsystem for a
        given configuration.
- [ ] `show flags <config-name>` prints the compiler flags specified by the buildsystem for
        a given configuration.
- [ ] `show metadata <project-path>` prints metadata for a project.
- [ ] `show structure <project-path>` prints the full structure of a project, starting from the toplevel
        one. The given project should be marked.

#### check

- [ ] `check config` displays whether the cmake_data.yaml is correct and works with the current project.
- [ ] `check cmake-version` gets the current CMake version and the required CMake version, and whether
        the current CMake version is new enough.

### Libraries I want to explicitly support for convenience

- Ideally anything listed in the [awesome-cpp repository](https://github.com/fffaraz/awesome-cpp) which either
  supports cross-platform CMake builds out of the box or is fairly easy to add. That repository is a fantastic
  list.

#### Pre-written CMake find modules

- [ ] Boost
- [ ] BZip2
- [x] CURL
- [ ] Doxygen
- [x] GLEW
- [ ] LibLZMA
- [ ] OpenAL-soft
- [ ] OpenMP
- [x] OpenGL
- [ ] OpenSceneGraph (maybe) (NOTE: has cmake package config file)
- [x] OpenSSL
- [ ] PNG
- [ ] PostgreSQL
- [x] SDL2
- [x] SQLite (3)
- [x] Threads
- [ ] TIFF
- [x] wxWidgets
- [ ] Vulkan
- [x] ZLIB

#### Other CMake projects

- [ ] [flux](https://github.com/tcbrindle/flux?tab=readme-ov-file) sequence-oriented programming library
- [ ] [cppcoro (maintained fork)](https://github.com/andreasbuhr/cppcoro)
- [x] [libwebsockets](https://github.com/warmcat/libwebsockets)
- [ ] [lpq++](https://github.com/jtv/libpqxx) PostgreSQL C++ client API
- [ ] [Qt6](https://www.qt.io/product/qt6)
- [x] [raylib](https://github.com/raysan5/raylib) (Also [works with WebAssembly](https://github.com/raysan5/raylib/wiki/Working-for-Web-(HTML5)))
- [x] [nlohmann json](https://github.com/nlohmann/json)
- [x] [SFML](https://www.sfml-dev.org/)
- [x] [fmt](https://github.com/fmtlib/fmt)
- [ ] [JUCE](https://juce.com/)
- [x] [yaml-cpp](https://github.com/jbeder/yaml-cpp)
- [x] [glfw3](https://www.glfw.org/)
- [ ] [OpenCV](https://opencv.org/)
- [ ] [ffmpeg](https://www.ffmpeg.org/)
- [ ] [TensorFlow](https://www.tensorflow.org/)
- [x] [GLM (OpenGL Mathematics)](https://github.com/g-truc/glm)
- [x] [cxxopts](https://github.com/jarro2783/cxxopts)
- [x] [CLI11](https://github.com/CLIUtils/CLI11)
- [x] [ftxui](https://github.com/ArthurSonzogni/FTXUI)
- [x] [pugixml](https://github.com/zeux/pugixml)
- [ ] [mimalloc](https://github.com/microsoft/mimalloc)
- [x] [magic_enum](https://github.com/Neargye/magic_enum)
- [x] [argparse](https://github.com/p-ranav/argparse)
- [x] [FreeType](https://freetype.org/index.html)
- [ ] [drogon](https://github.com/drogonframework/drogon) (This looks like it might take some work)
- [x] [re2](https://github.com/google/re2)
- [x] [zstd](https://github.com/facebook/zstd) CMake project, but use custom find module
- [ ] [Hyperscan](https://github.com/intel/hyperscan)
- [x] [Crow](https://github.com/CrowCpp/Crow)
- [ ] [libharu](https://github.com/libharu/libharu)
- [ ] [concurrencpp](https://github.com/David-Haim/concurrencpp)
- [ ] [opus](https://opus-codec.org/)
- [ ] [Vorbis](https://xiph.org/vorbis/)
- [ ] [glaze](https://github.com/stephenberry/glaze) JSON
- [ ] [smk](https://github.com/ArthurSonzogni/smk)

#### Support when FetchContent ready

- [ ] [GLM (The actual repo, not a fork)](https://github.com/g-truc/glm)

#### Testing Frameworks

- [x] [Catch2](https://github.com/catchorg/Catch2)
- [x] [doctest](https://github.com/doctest/doctest)
- [x] [GoogleTest](https://github.com/google/googletest)

#### Cryptography libraries

- [ ] [botan](https://github.com/randombit/botan)
- [ ] [crpytopp](https://github.com/weidai11/cryptopp)

#### Other projects

- [x] [stb](https://github.com/nothings/stb)
- [x] [imgui](https://github.com/ocornut/imgui)
- [x] [brotli](https://github.com/google/brotli)
