use std::sync::Arc;

use arrow::datatypes::DataType;
use arrow_array::cast::AsArray;
use geoarrow::array::metadata::ArrayMetadata;
use geoarrow::array::MixedGeometryArray;
use geoarrow::chunked_array::{ChunkedArray, ChunkedMixedGeometryArray};
use geoarrow::io::geozero::FromWKT;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3_arrow::input::AnyArray;

use crate::ffi::to_python::{chunked_geometry_array_to_pyobject, geometry_array_to_pyobject};
use pyo3_geoarrow::{PyCoordType, PyGeoArrowResult};

#[pyfunction]
#[pyo3(
    signature = (input, *, coord_type = PyCoordType::Interleaved),
    text_signature = "(input, *, method = 'interleaved')")
]
pub fn from_wkt(
    py: Python,
    input: AnyArray,
    coord_type: PyCoordType,
) -> PyGeoArrowResult<PyObject> {
    let coord_type = coord_type.into();
    match input {
        AnyArray::Array(arr) => {
            let (array, field) = arr.into_inner();
            let metadata = Arc::new(ArrayMetadata::try_from(field.as_ref())?);
            let geo_array: MixedGeometryArray<i32, 2> = match array.data_type() {
                DataType::Utf8 => {
                    FromWKT::from_wkt(array.as_string::<i32>(), coord_type, metadata, false)?
                }
                DataType::LargeUtf8 => {
                    FromWKT::from_wkt(array.as_string::<i64>(), coord_type, metadata, false)?
                }
                other => {
                    return Err(
                        PyTypeError::new_err(format!("Unexpected array type {:?}", other)).into(),
                    )
                }
            };
            geometry_array_to_pyobject(py, Arc::new(geo_array))
        }
        AnyArray::Stream(s) => {
            let chunked_arr = s.into_chunked_array()?;
            let (chunks, field) = chunked_arr.into_inner();
            let metadata = Arc::new(ArrayMetadata::try_from(field.as_ref())?);
            let geo_array: ChunkedMixedGeometryArray<i32, 2> = match field.data_type() {
                DataType::Utf8 => {
                    let string_chunks = chunks
                        .iter()
                        .map(|chunk| chunk.as_string::<i32>().clone())
                        .collect::<Vec<_>>();
                    FromWKT::from_wkt(
                        &ChunkedArray::new(string_chunks),
                        coord_type,
                        metadata,
                        false,
                    )?
                }
                DataType::LargeUtf8 => {
                    let string_chunks = chunks
                        .iter()
                        .map(|chunk| chunk.as_string::<i64>().clone())
                        .collect::<Vec<_>>();
                    FromWKT::from_wkt(
                        &ChunkedArray::new(string_chunks),
                        coord_type,
                        metadata,
                        false,
                    )?
                }
                other => {
                    return Err(
                        PyTypeError::new_err(format!("Unexpected array type {:?}", other)).into(),
                    )
                }
            };
            chunked_geometry_array_to_pyobject(py, Arc::new(geo_array))
        }
    }
}
