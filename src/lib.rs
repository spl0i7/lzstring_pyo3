use pyo3::prelude::{pyfunction, pymodule, PyResult, PyErr, Python, PyModule};
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
fn lzma_pyo3(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compressToBase64, m)?)?;
    m.add_function(wrap_pyfunction!(decompressFromBase64, m)?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use lz_str::compress_to_base64;
    use lz_str::decompress_from_base64;

    #[test]
    fn test_compress_decompress() {
        let sentence = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
        let split_sentence: Vec<&str> = sentence.split(" ").collect();

        let compressed: Vec<String> = split_sentence.iter().
            map(|x| compress_to_base64(x)).
            collect();


        assert_eq!(compressed.len(), split_sentence.len());

        let decompressed: Vec<String> = compressed.
            iter().
            map(|x| decompress_from_base64(x.as_str())).
            filter(|x| x.is_some()).
            map(|x| x.unwrap()).
            map(|x| String::from_utf16(&*x)).
            filter(|x| x.is_ok()).
            map(|x| x.unwrap())
            .collect();

        assert_eq!(decompressed.len(), split_sentence.len());


        decompressed.iter().zip(split_sentence).for_each(|x| {
            assert_eq!(x.1, x.0)
        })



    }
}
