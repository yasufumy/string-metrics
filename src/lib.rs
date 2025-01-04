mod distance;
use pyo3::prelude::*;

use crate::distance::levenshtein;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn text_string_metrics(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    Ok(())
}
