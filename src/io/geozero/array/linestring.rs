use arrow_array::OffsetSizeTrait;
use geozero::{GeomProcessor, GeozeroGeometry};

use crate::array::linestring::LineStringCapacity;
use crate::array::{LineStringArray, LineStringBuilder};
use crate::io::geozero::scalar::process_line_string;
use crate::trait_::NativeArrayAccessor;
use crate::NativeArray;

impl<O: OffsetSizeTrait, const D: usize> GeozeroGeometry for LineStringArray<O, D> {
    fn process_geom<P: GeomProcessor>(&self, processor: &mut P) -> geozero::error::Result<()>
    where
        Self: Sized,
    {
        let num_geometries = self.len();
        processor.geometrycollection_begin(num_geometries, 0)?;

        for geom_idx in 0..num_geometries {
            process_line_string(&self.value(geom_idx), geom_idx, processor)?;
        }

        processor.geometrycollection_end(num_geometries - 1)?;
        Ok(())
    }
}

/// GeoZero trait to convert to GeoArrow LineStringArray.
pub trait ToLineStringArray<O: OffsetSizeTrait, const D: usize> {
    /// Convert to GeoArrow LineStringArray
    fn to_line_string_array(&self) -> geozero::error::Result<LineStringArray<O, D>>;

    /// Convert to a GeoArrow LineStringBuilder
    fn to_line_string_builder(&self) -> geozero::error::Result<LineStringBuilder<O, D>>;
}

impl<T: GeozeroGeometry, O: OffsetSizeTrait, const D: usize> ToLineStringArray<O, D> for T {
    fn to_line_string_array(&self) -> geozero::error::Result<LineStringArray<O, D>> {
        Ok(self.to_line_string_builder()?.into())
    }

    fn to_line_string_builder(&self) -> geozero::error::Result<LineStringBuilder<O, D>> {
        let mut mutable_array = LineStringBuilder::new();
        self.process_geom(&mut mutable_array)?;
        Ok(mutable_array)
    }
}

#[allow(unused_variables)]
impl<O: OffsetSizeTrait, const D: usize> GeomProcessor for LineStringBuilder<O, D> {
    fn geometrycollection_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        let capacity = LineStringCapacity::new(0, size);
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
        unsafe { self.push_coord(&geo::Coord { x, y }) }
        Ok(())
    }

    fn linestring_begin(
        &mut self,
        tagged: bool,
        size: usize,
        idx: usize,
    ) -> geozero::error::Result<()> {
        let capacity = LineStringCapacity::new(size, 0);
        self.reserve(capacity);
        self.try_push_length(size).unwrap();
        Ok(())
    }

    fn linestring_end(&mut self, tagged: bool, idx: usize) -> geozero::error::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::linestring::{ls0, ls1};
    use crate::trait_::NativeArrayAccessor;
    use geo::Geometry;
    use geozero::error::Result;
    use geozero::ToWkt;

    #[test]
    fn geozero_process_geom() -> geozero::error::Result<()> {
        let arr: LineStringArray<i64, 2> = vec![ls0(), ls1()].as_slice().into();
        let wkt = arr.to_wkt()?;
        let expected = "GEOMETRYCOLLECTION(LINESTRING(0 1,1 2),LINESTRING(3 4,5 6))";
        assert_eq!(wkt, expected);
        Ok(())
    }

    #[test]
    fn from_geozero() -> Result<()> {
        let geo = Geometry::GeometryCollection(
            vec![ls0(), ls1()]
                .into_iter()
                .map(Geometry::LineString)
                .collect(),
        );
        let multi_point_array: LineStringArray<i32, 2> = geo.to_line_string_array().unwrap();
        assert_eq!(multi_point_array.value_as_geo(0), ls0());
        assert_eq!(multi_point_array.value_as_geo(1), ls1());
        Ok(())
    }
}
