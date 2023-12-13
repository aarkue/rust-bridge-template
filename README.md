# Rust Bridge Template

This repository serves as a template to easily start developing a shared library in __Rust__ with __bindings__ to __Python__ and __Java__.


## Getting Started
1. Clone this repository template
2. Install Rust (see https://www.rust-lang.org/tools/install and https://www.rust-lang.org/learn/get-started)
3. Build all Rust projects parts by executing `cargo build --release`
4. Try running any of the sample programs or bindings listed below


## Parts & How to Run them
- `main_library` is the main Rust library with all the shared functionality
  - This library is used and automatically compiled in any of the following use cases
  - You can also compile it manually by navigating to `main_library/` and executing `cargo build --release`
- `executable` is a Rust binary (meaning it can be executed directly)
  - Navigating to the `executable/` directory and running `cargo run --release`
- `java_bridge` is a Rust library with Java bindings. It can be used from Java programs using JNI.
  - Navigate to the `java_bridge/` directory, build the library using `cargo build --release`
  - Next, enter the `java_wrapper/` subdirectory and run `javac RustBridge.java`
  - Finally, run `java -Djava.library.path=../../target/release/ RustBridge` to execute the compiled Java program 
- `python_bridge` is a Rust library with Python bindings. It can be used to easily create a Python package with native Rust parts
  - Navigate to the `python_bridge/` directory, create a virtual environment (e.g., `python -m venv .env`) and activate it
  - Install `maturin` (https://www.maturin.rs/) in this environment (`pip install maturin`)
  - Finally, develop build and install the python library using `maturin develop --release` and then execute the `test.py` file using `python ./python_wrapper/test.py`