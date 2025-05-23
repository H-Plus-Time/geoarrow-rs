use geoarrow_schema::{CoordType, Dimension};
use geozero::GeozeroDatasource;
use geozero::geojson::GeoJsonReader;
use std::io::Read;

use crate::error::Result;
use crate::io::geozero::array::GeometryStreamBuilder;
use crate::io::geozero::table::{GeoTableBuilder, GeoTableBuilderOptions};
use crate::table::Table;

/// Read a GeoJSON file to a Table.
pub fn read_geojson<R: Read>(reader: R, batch_size: Option<usize>) -> Result<Table> {
    let mut geojson = GeoJsonReader(reader);
    // TODO: set CRS to epsg:4326?
    let options = GeoTableBuilderOptions::new(
        CoordType::Interleaved,
        true,
        batch_size,
        None,
        None,
        Default::default(),
    );
    let mut geo_table =
        GeoTableBuilder::<GeometryStreamBuilder>::new_with_options(Dimension::XY, options);
    geojson.process(&mut geo_table)?;
    geo_table.finish()
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;

    use super::*;

    #[ignore = "non-vendored file"]
    #[test]
    fn test_read_geojson() {
        let path = "/Users/kyle/Downloads/UScounties.geojson";
        let mut filein = BufReader::new(File::open(path).unwrap());
        let _table = read_geojson(&mut filein, None).unwrap();
    }
}
