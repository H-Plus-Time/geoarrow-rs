use crate::error::PyGeoArrowResult;
use crate::io::input::sync::{FileReader, FileWriter};
use crate::util::table_to_pytable;
use geoarrow::io::csv::read_csv as _read_csv;
use geoarrow::io::csv::write_csv as _write_csv;
use geoarrow::io::csv::CSVReaderOptions;
use pyo3::prelude::*;
use pyo3_arrow::input::AnyRecordBatch;

#[pyfunction]
#[pyo3(signature = (file, geometry_column_name, *, batch_size=65536))]
pub fn read_csv(
    py: Python,
    mut file: FileReader,
    geometry_column_name: &str,
    batch_size: usize,
) -> PyGeoArrowResult<PyObject> {
    let options = CSVReaderOptions::new(Default::default(), batch_size);
    let table = _read_csv(&mut file, geometry_column_name, options)?;
    Ok(table_to_pytable(table).to_arro3(py)?)
}

#[pyfunction]
#[pyo3(signature = (table, file))]
pub fn write_csv(table: AnyRecordBatch, file: FileWriter) -> PyGeoArrowResult<()> {
    _write_csv(table.into_reader()?, file)?;
    Ok(())
}
