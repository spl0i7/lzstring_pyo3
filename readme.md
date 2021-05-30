# lzma_pyo3

## Installation

- Install Rust using [RustUp](https://rustup.rs/)
  
- Install Maturin for packaging which can build and publish crates with pyo3, rust-cpython and cffi bindings as well as rust binaries as python packages. `pip install maturin`
  
- Update pip (old pip creates problems when used with Maturin). `pip3 install --upgrade pip`
  
- Clone this repo `git clone https://github.com/spl0i7/lzstring_pyo3` and goto the directory.
  
- Build with Maturin `maturin build`

- Install build package with `pip3 install <path/to/whl>`