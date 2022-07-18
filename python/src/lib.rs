#![allow(clippy::deprecated)]
use pyo3::prelude::*;
use tokenizations::{
    get_alignments as get_alignments_orig, get_charmap as get_charmap_orig, Alignment, CharMap,
};

#[pyfunction]
pub fn get_alignments(a: Vec<&str>, b: Vec<&str>) -> PyResult<(Alignment, Alignment)> {
    Ok(get_alignments_orig(&a, &b))
}

#[pyfunction]
pub fn get_charmap(a: &str, b: &str) -> PyResult<(CharMap, CharMap)> {
    Ok(get_charmap_orig(a, b))
}

#[pymodule]
fn tokenizations(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", "0.8.4")?;
    m.add_function(wrap_pyfunction!(get_alignments, m)?)?;
    m.add_function(wrap_pyfunction!(get_charmap, m)?)?;

    Ok(())
}
