use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use tokenizations::{get_alignments, Alignment};

#[pymodule]
fn tokenizations(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "get_alignments")]
    pub fn get_alignments_py(
        _py: Python,
        a: Vec<&str>,
        b: Vec<&str>,
    ) -> PyResult<(Alignment, Alignment)> {
        Ok(get_alignments(&a, &b))
    }

    Ok(())
}
