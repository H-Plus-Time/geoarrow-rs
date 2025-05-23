use std::sync::Arc;

use geoarrow::array::{
    LineStringArray, MultiLineStringArray, MultiPointArray, MultiPolygonArray, NativeArrayDyn,
    PointArray, PolygonArray,
};
use geoarrow_schema::Metadata;
use pyo3::prelude::*;
use pyo3_geoarrow::{PyCoordBuffer, PyCrs, PyGeoArrowResult, PyNativeArray, PyOffsetBuffer};

fn create_array_metadata(crs: Option<PyCrs>) -> Arc<Metadata> {
    Arc::new(crs.map(|inner| inner.into_inner()).unwrap_or_default())
}

#[pyfunction]
#[pyo3(signature = (coords, *, crs = None))]
pub fn points(coords: PyCoordBuffer, crs: Option<PyCrs>) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    let array = PointArray::new(coords.into_inner(), None, metadata);
    Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, *, crs = None))]
pub fn linestrings(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    crs: Option<PyCrs>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    let array = LineStringArray::new(
        coords.into_inner(),
        geom_offsets.into_inner(),
        None,
        metadata,
    );
    Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, ring_offsets, *, crs = None))]
pub fn polygons(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    ring_offsets: PyOffsetBuffer,
    crs: Option<PyCrs>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    let array = PolygonArray::new(
        coords.into_inner(),
        geom_offsets.into_inner(),
        ring_offsets.into_inner(),
        None,
        metadata,
    );
    Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, *, crs = None))]
pub fn multipoints(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    crs: Option<PyCrs>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    let array = MultiPointArray::new(
        coords.into_inner(),
        geom_offsets.into_inner(),
        None,
        metadata,
    );
    Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, ring_offsets, *, crs = None))]
pub fn multilinestrings(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    ring_offsets: PyOffsetBuffer,
    crs: Option<PyCrs>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    let array = MultiLineStringArray::new(
        coords.into_inner(),
        geom_offsets.into_inner(),
        ring_offsets.into_inner(),
        None,
        metadata,
    );
    Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
}

#[pyfunction]
#[pyo3(signature = (coords, geom_offsets, polygon_offsets, ring_offsets, *, crs = None))]
pub fn multipolygons(
    coords: PyCoordBuffer,
    geom_offsets: PyOffsetBuffer,
    polygon_offsets: PyOffsetBuffer,
    ring_offsets: PyOffsetBuffer,
    crs: Option<PyCrs>,
) -> PyGeoArrowResult<PyNativeArray> {
    let metadata = create_array_metadata(crs);
    let array = MultiPolygonArray::new(
        coords.into_inner(),
        geom_offsets.into_inner(),
        polygon_offsets.into_inner(),
        ring_offsets.into_inner(),
        None,
        metadata,
    );
    Ok(PyNativeArray::new(NativeArrayDyn::new(Arc::new(array))))
}
