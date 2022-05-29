mod ffi;
pub mod geodataframe;

use arctic::geoseries::GeoSeries;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use crate::geodataframe::{BaseClass, SubClass, SubSubClass};

#[pyfunction]
fn centroid(series: &PyAny) -> PyResult<PyObject> {
    let series = ffi::py_series_to_rust_series(series)?;
    let out = series
        .centroid()
        .map_err(|e| PyValueError::new_err(format!("Something went wrong: {:?}", e)))?;
    ffi::rust_series_to_py_series(&out)
}

#[pymodule]
fn py_arctic(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(centroid))?;
    m.add_class::<BaseClass>()?;
    m.add_class::<SubClass>()?;
    m.add_class::<SubSubClass>()?;
    Ok(())
}