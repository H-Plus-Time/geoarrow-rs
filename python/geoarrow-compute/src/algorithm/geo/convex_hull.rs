use std::sync::Arc;

use crate::ffi::from_python::AnyGeometryInput;
use crate::util::{return_chunked_geometry_array, return_geometry_array};
use geoarrow::algorithm::geo::ConvexHull;
use geoarrow::array::PolygonArray;
use geoarrow::chunked_array::ChunkedGeometryArray;
use pyo3::prelude::*;
use pyo3_geoarrow::PyGeoArrowResult;

#[pyfunction]
pub fn convex_hull(py: Python, input: AnyGeometryInput) -> PyGeoArrowResult<PyObject> {
    match input {
        AnyGeometryInput::Array(arr) => {
            let out: PolygonArray<i32, 2> = arr.as_ref().convex_hull()?;
            return_geometry_array(py, Arc::new(out))
        }
        AnyGeometryInput::Chunked(arr) => {
            let out: ChunkedGeometryArray<PolygonArray<i32, 2>> = arr.as_ref().convex_hull()?;
            return_chunked_geometry_array(py, Arc::new(out))
        }
    }
}
