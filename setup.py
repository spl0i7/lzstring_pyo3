#!/usr/bin/env python
import sys

from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="lzma-pyo3",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Rust",
        "Operating System :: POSIX"
    ],
    packages=["lzma_pyo3"],
    rust_extensions=[RustExtension("lzma_pyo3.lzma_pyo3")],
    include_package_data=True,
    zip_safe=False,
)