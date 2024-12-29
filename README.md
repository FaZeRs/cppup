# cppup

A powerful and interactive C++ project generator written in Rust. It helps you quickly set up new C++ projects with modern best practices and your preferred configurations.

## Features

- 🎯 Interactive CLI with smart defaults
- 🏗️ Multiple build systems (CMake, Make)
- 📦 Package manager integration (Conan, Vcpkg)
- ✅ Testing framework setup (doctest, Google Test, Catch2, Boost.Test)
- 🔍 Code quality tools (clang-format, clang-tidy, cppcheck)
- 📝 License management (MIT, Apache-2.0, GPL-3.0, BSD-3-Clause)
- 🎨 Project templates (Executable, Library)
- 🔄 Git initialization

## Prerequisites

- C++ compiler - clang or gcc
- CMake or Make build system
- Optional: Conan or Vcpkg package manager
- Optional: clang-format, clang-tidy, or cppcheck for code quality tools

## Building

```bash
cargo build
```

## Usage

### Interactive Mode

Simply run:

```bash
cppup
```

Follow the interactive prompts to configure your project.

### Non-Interactive Mode

Create a new executable project with specific settings:

```bash
cppup --name my-project \
      --description "My awesome C++ project" \
      --project-type executable \
      --build-system cmake \
      --cpp-standard 17 \
      --package-manager conan \
      --test-framework doctest \
      --license MIT \
      --quality-tools clang-format,clang-tidy \
      --non-interactive
```

### Available Options

- `--name`: Project name
- `--description`: Project description
- `--project-type`: `executable` or `library`
- `--build-system`: `cmake` or `make`
- `--cpp-standard`: `11`, `14`, `17`, `20`, or `23`
- `--package-manager`: `none`, `conan`, or `vcpkg`
- `--test-framework`: `none`, `doctest`, `gtest`, `catch2`, or `boosttest`
- `--license`: `MIT`, `Apache-2.0`, `GPL-3.0`, or `BSD-3-Clause`
- `--quality-tools`: Comma-separated list of `clang-format`, `clang-tidy`, `cppcheck`
- `--non-interactive`: Skip interactive prompts
- `--path`: Output directory (default: current directory)
- `--git`: Initialize git repository (default: true)

## Project Structure

Generated project structure for an executable:

```
my-project/
├── src/
│   └── main.cpp
├── include/
├── assets/
├── tests/           # If testing is enabled
├── build/
├── CMakeLists.txt   # Or Makefile
├── .gitignore
├── LICENSE
└── README.md
```

For a library:

```
my-project/
├── src/
│   └── lib.cpp
├── include/
│   └── my-project.hpp
├── examples/
├── tests/           # If testing is enabled
├── build/
├── CMakeLists.txt   # Or Makefile
├── .gitignore
├── LICENSE
└── README.md
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Nauris Linde <naurislinde@gmail.com>

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Acknowledgments

- [Rust](https://www.rust-lang.org/) for the programming language
- [Handlebars](https://handlebarsjs.com/) for templating
- [Conan](https://conan.io/) for package management
- [Vcpkg](https://github.com/microsoft/vcpkg) for package management
- [CMake](https://cmake.org/) for build system
- [Make](https://www.gnu.org/software/make/) for build system
- [doctest](https://github.com/doctest/doctest) for testing
- [Catch2](https://github.com/catchorg/Catch2) for testing
- [Boost.Test](https://www.boost.org/doc/libs/1_83_0/libs/test/doc/html/index.html) for testing
- [Google Test](https://github.com/google/googletest) for testing
- [Clang Tidy](https://clang.llvm.org/extra/clang-tidy/) for code quality
- [Cppcheck](https://cppcheck.sourceforge.io/) for code quality
- [Clang Format](https://clang.llvm.org/docs/ClangFormat.html) for code formatting
