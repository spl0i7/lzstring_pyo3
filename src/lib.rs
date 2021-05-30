use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions;

#[pyfunction]
fn compressToBase64(input: String) -> PyResult<String> {
    Ok(lz_str::compress_to_base64(input.as_str()))
}

#[pyfunction]
fn decompressFromBase64(input: String) -> PyResult<String> {
    let result = lz_str::decompress_from_base64(input.as_str());
    if let None = result {
        return Err(exceptions::PyTypeError::new_err("decompression failed"));
    }
    let to_string = String::from_utf16(&*result.unwrap());

    if let Err(e) = to_string {
        return Err(exceptions::PyTypeError::new_err(e.to_string()));
    }


    Ok(to_string.unwrap())
}


/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compressToBase64, m)?)?;
    m.add_function(wrap_pyfunction!(decompressFromBase64, m)?)?;
    Ok(())
}