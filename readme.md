[![Build Module](https://github.com/spl0i7/lzstring_pyo3/actions/workflows/rust.yml/badge.svg)](https://github.com/spl0i7/lzstring_pyo3/actions/workflows/rust.yml)

# lzma_pyo3

Python module based on the [LZ-String javascript]( http://pieroxy.net/blog/pages/lz-string/index.html), purpose here is to be faster than existing native python [implementation](https://pypi.org/project/lzstring/)

## Installation

- Install Rust using [RustUp](https://rustup.rs/)
  
- Install Maturin for packaging which can build and publish crates with pyo3, rust-cpython and cffi bindings as well as rust binaries as python packages. `pip install maturin`
  
- Update pip (old pip creates problems when used with Maturin). `pip3 install --upgrade pip`
  
- Clone this repo `git clone https://github.com/spl0i7/lzstring_pyo3` and goto the directory.
  
- Build with Maturin `maturin build`

- Install build package with `pip3 install <path/to/whl>`

## Benchmark

```python

import timeit
import random
import string

import lzstring
from lzma_pyo3 import compressToBase64, decompressFromBase64

N=1000
SEQ = ''.join(random.choices(string.ascii_uppercase + string.digits, k=N))

X = lzstring.LZString()

r1 = timeit.timeit("""
from __main__ import SEQ, X
a = X.compressToBase64(SEQ)
b = X.decompressFromBase64(a)
""", number=100)

print("pure py: ", r1)

r2 = timeit.timeit("""
from __main__ import SEQ, compressToBase64, decompressFromBase64
a = compressToBase64(SEQ)
b = decompressFromBase64(a)
""", number=100)

print("rust-py: ", r2)

```
```
pure py:  1.3936761820077663
rust-py:  0.0515352350048488
```

## Caveats

`lzstring` and `lzma_pyo3` can produce different outputs when `compressToBase64` is called. This is because of different base64 padding. 

```python
import lzstring
import lzma_pyo3

X = lzstring.LZString()
print(X.compressToBase64('hello'))
print(lzma_pyo3.compressToBase64('hello'))

```
```
BYUwNmD2Q===
BYUwNmD2Q====
```

Since this is just related to base64 padding, it does not mean much. `decompressFromBase64` from both package can decompress either of the strings.  
