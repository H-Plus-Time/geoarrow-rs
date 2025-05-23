use geozero::{GeomProcessor, GeozeroGeometry};

use crate::ArrayBase;
use crate::array::multilinestring::MultiLineStringCapacity;
use crate::array::{MultiLineStringArray, MultiLineStringBuilder};
use crate::io::geozero::scalar::process_multi_line_string;
use crate::trait_::ArrayAccessor;
use geoarrow_schema::Dimension;

impl GeozeroGeometry for MultiLineStringArray {
    fn process_geom<P: GeomProcessor>(&self, processor: &mut P) -> geozero::error::Result<()>
    where
        Self: Sized,
    {
        let num_geometries = self.len();
        processor.geometrycollection_begin(num_geometries, 0)?;

        for geom_idx in 0..num_geometries {
            process_multi_line_string(&self.value(geom_idx), geom_idx, processor)?;
        }

        processor.geometrycollection_end(num_geometries - 1)?;
        Ok(())
    }
}

/// GeoZero trait to convert to GeoArrow MultiLineStringArray.
pub trait ToMultiLineStringArray {
    /// Convert to GeoArrow MultiLineStringArray
    fn to_multi_line_string_array(
        &self,
        dim: Dimension,
    ) -> geozero::error::Result<MultiLineStringArray>;

    /// Convert to a GeoArrow MultiLineStringBuilder
    fn to_multi_line_string_builder(
        &self,
        dim: Dimension,
    ) -> geozero::error::Result<MultiLineStringBuilder>;
}

impl<T: GeozeroGeometry> ToMultiLineStringArray for T {
    fn to_multi_line_string_array(
        &self,
        dim: Dimension,
    ) -> geozero::error::Result<MultiLineStringArray> {
        Ok(self.to_multi_line_string_builder(dim)?.into())
    }

    fn to_multi_line_string_builder(
        &self,
        dim: Dimension,
    ) -> geozero::error::Result<MultiLineStringBuilder> {
        let mut mutable_array = MultiLineStringBuilder::new(dim);
        self.process_geom(&mut mutable_array)?;
        Ok(mutable_array)
    }
}

#[allow(unused_variables)]
impl GeomProcessor for MultiLineStringBuilder {
    fn geometrycollection_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        // reserve `size` geometries
        let capacity = MultiLineStringCapacity::new(0, 0, size);
        self.reserve(capacity);
        Ok(())
    }

    fn geometrycollection_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        // self.shrink_to_fit()
        Ok(())
    }

    fn xy(&mut self, x: f64, y: f64, idx: usize) -> geozero::error::Result<()> {
        // # Safety:
        // This upholds invariants because we call try_push_length in multipoint_begin to ensure
        // offset arrays are correct.
        unsafe { self.push_coord(&geo::Coord { x, y }).unwrap() }
        Ok(())
    }

    // Here, size is the number of LineStrings in the MultiLineString
    fn multilinestring_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        // reserve `size` line strings
        let capacity = MultiLineStringCapacity::new(0, size, 0);
        self.reserve(capacity);

        // # Safety:
        // This upholds invariants because we separately update the ring offsets in
        // linestring_begin
        unsafe { self.try_push_geom_offset(size).unwrap() }
        Ok(())
    }

    fn linestring_begin(
        &mut self,
        tagged: bool,
        size: usize,
        idx: usize,
    ) -> geozero::error::Result<()> {
        // > An untagged LineString is either a Polygon ring or part of a MultiLineString
        // So if tagged, we need to update the geometry offsets array.
        if tagged {
            // reserve 1 line strings
            let capacity = MultiLineStringCapacity::new(0, 1, 0);
            self.reserve(capacity);

            // # Safety:
            // This upholds invariants because we separately update the ring offsets in
            // linestring_begin
            unsafe { self.try_push_geom_offset(1).unwrap() }
        }

        // reserve `size` coordinates
        let capacity = MultiLineStringCapacity::new(size, 0, 0);
        self.reserve(capacity);

        // # Safety:
        // This upholds invariants because we separately update the geometry offsets in
        // polygon_begin
        unsafe { self.try_push_ring_offset(size).unwrap() }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::multilinestring::{ml0, ml1};
    use crate::trait_::ArrayAccessor;
    use geo::Geometry;
    use geozero::ToWkt;
    use geozero::error::Result;

    #[test]
    fn geozero_process_geom() -> geozero::error::Result<()> {
        let arr: MultiLineStringArray = (vec![ml0(), ml1()].as_slice(), Dimension::XY).into();
        let wkt = arr.to_wkt()?;
        let expected = "GEOMETRYCOLLECTION(MULTILINESTRING((-111 45,-111 41,-104 41,-104 45)),MULTILINESTRING((-111 45,-111 41,-104 41,-104 45),(-110 44,-110 42,-105 42,-105 44)))";
        assert_eq!(wkt, expected);
        Ok(())
    }

    #[test]
    fn from_geozero() -> Result<()> {
        let geo = Geometry::GeometryCollection(
            vec![ml0(), ml1()]
                .into_iter()
                .map(Geometry::MultiLineString)
                .collect(),
        );
        let multi_point_array = geo.to_multi_line_string_array(Dimension::XY).unwrap();
        assert_eq!(multi_point_array.value_as_geo(0), ml0());
        assert_eq!(multi_point_array.value_as_geo(1), ml1());
        Ok(())
    }
}
