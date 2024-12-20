use geoarrow::datatypes::Dimension;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum PyDimension {
    XY,
    XYZ,
}

impl<'a> FromPyObject<'a> for PyDimension {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let s: String = ob.extract()?;
        match s.to_lowercase().as_str() {
            "xy" => Ok(Self::XY),
            "xyz" => Ok(Self::XYZ),
            _ => Err(PyValueError::new_err("Unexpected dimension")),
        }
    }
}

impl From<PyDimension> for Dimension {
    fn from(value: PyDimension) -> Self {
        match value {
            PyDimension::XY => Self::XY,
            PyDimension::XYZ => Self::XYZ,
        }
    }
}
