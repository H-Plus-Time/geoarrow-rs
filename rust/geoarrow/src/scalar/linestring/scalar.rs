use crate::algorithm::native::bounding_rect::bounding_rect_linestring;
use crate::algorithm::native::eq::line_string_eq;
use crate::array::CoordBuffer;
use crate::array::util::OffsetBufferUtils;
use crate::scalar::Coord;
use crate::trait_::NativeScalar;
use arrow_buffer::OffsetBuffer;
use geo_traits::LineStringTrait;
use geo_traits::to_geo::ToGeoLineString;
use rstar::{AABB, RTreeObject};

/// An Arrow equivalent of a LineString
///
/// This implements [LineStringTrait], which you can use to extract data.
#[derive(Debug, Clone)]
pub struct LineString<'a> {
    pub(crate) coords: &'a CoordBuffer,

    /// Offsets into the coordinate array where each geometry starts
    pub(crate) geom_offsets: &'a OffsetBuffer<i32>,

    pub(crate) geom_index: usize,

    start_offset: usize,
}

impl<'a> LineString<'a> {
    pub(crate) fn new(
        coords: &'a CoordBuffer,
        geom_offsets: &'a OffsetBuffer<i32>,
        geom_index: usize,
    ) -> Self {
        let (start_offset, _) = geom_offsets.start_end(geom_index);
        Self {
            coords,
            geom_offsets,
            geom_index,
            start_offset,
        }
    }

    pub(crate) fn into_owned_inner(self) -> (CoordBuffer, OffsetBuffer<i32>, usize) {
        (
            self.coords.clone(),
            self.geom_offsets.clone(),
            self.geom_index,
        )
    }
}

impl NativeScalar for LineString<'_> {
    type ScalarGeo = geo::LineString;

    fn to_geo(&self) -> Self::ScalarGeo {
        self.into()
    }

    fn to_geo_geometry(&self) -> geo::Geometry {
        geo::Geometry::LineString(self.to_geo())
    }

    #[cfg(feature = "geos")]
    fn to_geos(&self) -> std::result::Result<geos::Geometry, geos::Error> {
        self.try_into()
    }
}

impl<'a> LineStringTrait for LineString<'a> {
    type T = f64;
    type CoordType<'b>
        = Coord<'a>
    where
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        self.coords.dim().into()
    }

    fn num_coords(&self) -> usize {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        end - start
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        self.coords.value(self.start_offset + i)
    }
}

impl<'a> LineStringTrait for &'a LineString<'a> {
    type T = f64;
    type CoordType<'b>
        = Coord<'a>
    where
        Self: 'b;

    fn dim(&self) -> geo_traits::Dimensions {
        self.coords.dim().into()
    }

    fn num_coords(&self) -> usize {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        end - start
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        self.coords.value(self.start_offset + i)
    }
}

impl From<LineString<'_>> for geo::LineString {
    fn from(value: LineString<'_>) -> Self {
        (&value).into()
    }
}

impl From<&LineString<'_>> for geo::LineString {
    fn from(value: &LineString<'_>) -> Self {
        value.to_line_string()
    }
}

impl From<LineString<'_>> for geo::Geometry {
    fn from(value: LineString<'_>) -> Self {
        geo::Geometry::LineString(value.into())
    }
}

impl RTreeObject for LineString<'_> {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let (lower, upper) = bounding_rect_linestring(self);
        AABB::from_corners(lower, upper)
    }
}

impl<G: LineStringTrait<T = f64>> PartialEq<G> for LineString<'_> {
    fn eq(&self, other: &G) -> bool {
        line_string_eq(self, other)
    }
}

#[cfg(test)]
mod test {
    use crate::array::LineStringArray;
    use crate::test::linestring::{ls0, ls1};
    use crate::trait_::ArrayAccessor;
    use geoarrow_schema::Dimension;

    /// Test Eq where the current index is true but another index is false
    #[test]
    fn test_eq_other_index_false() {
        let arr1: LineStringArray = (vec![ls0(), ls1()].as_slice(), Dimension::XY).into();
        let arr2: LineStringArray = (vec![ls0(), ls0()].as_slice(), Dimension::XY).into();

        assert_eq!(arr1.value(0), arr2.value(0));
        assert_ne!(arr1.value(1), arr2.value(1));
    }
}
