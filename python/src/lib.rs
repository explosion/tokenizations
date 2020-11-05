#![allow(clippy::deprecated)]
use pyo3::prelude::*;
use tokenizations::{get_alignments, get_charmap, get_original_spans, Alignment, CharMap};

#[pymodule]
fn tokenizations(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", "0.7.2")?;

    #[pyfn(m, "get_alignments")]
    pub fn get_alignments_py(
        _py: Python,
        a: Vec<&str>,
        b: Vec<&str>,
    ) -> PyResult<(Alignment, Alignment)> {
        Ok(get_alignments(&a, &b))
    }

    #[pyfn(m, "get_charmap")]
    pub fn get_charmap_py(_py: Python, a: &str, b: &str) -> PyResult<(CharMap, CharMap)> {
        Ok(get_charmap(a, b))
    }

    #[pyfn(m, "get_original_spans")]
    pub fn get_original_spans_py(
        _py: Python,
        tokens: Vec<&str>,
        original_text: &str,
    ) -> PyResult<Vec<Option<(usize, usize)>>> {
        Ok(get_original_spans(&tokens, original_text))
    }

    Ok(())
}
