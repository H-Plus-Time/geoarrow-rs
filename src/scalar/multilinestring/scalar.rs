use crate::algorithm::native::bounding_rect::bounding_rect_multilinestring;
use crate::array::CoordBuffer;
use crate::geo_traits::MultiLineStringTrait;
use crate::scalar::multilinestring::MultiLineStringIterator;
use crate::scalar::LineString;
use crate::trait_::GeometryScalarTrait;
use crate::GeometryArrayTrait;
use arrow2::offset::OffsetsBuffer;
use arrow2::types::Offset;
use rstar::{RTreeObject, AABB};
use std::borrow::Cow;

/// An Arrow equivalent of a MultiLineString
#[derive(Debug, Clone)]
pub struct MultiLineString<'a, O: Offset> {
    pub coords: Cow<'a, CoordBuffer>,

    /// Offsets into the ring array where each geometry starts
    pub geom_offsets: Cow<'a, OffsetsBuffer<O>>,

    /// Offsets into the coordinate array where each ring starts
    pub ring_offsets: Cow<'a, OffsetsBuffer<O>>,

    pub geom_index: usize,
}

impl<'a, O: Offset> MultiLineString<'a, O> {
    pub fn new(
        coords: Cow<'a, CoordBuffer>,
        geom_offsets: Cow<'a, OffsetsBuffer<O>>,
        ring_offsets: Cow<'a, OffsetsBuffer<O>>,
        geom_index: usize,
    ) -> Self {
        Self {
            coords,
            geom_offsets,
            ring_offsets,
            geom_index,
        }
    }

    pub fn new_borrowed(
        coords: &'a CoordBuffer,
        geom_offsets: &'a OffsetsBuffer<O>,
        ring_offsets: &'a OffsetsBuffer<O>,
        geom_index: usize,
    ) -> Self {
        Self {
            coords: Cow::Borrowed(coords),
            geom_offsets: Cow::Borrowed(geom_offsets),
            ring_offsets: Cow::Borrowed(ring_offsets),
            geom_index,
        }
    }

    pub fn new_owned(
        coords: CoordBuffer,
        geom_offsets: OffsetsBuffer<O>,
        ring_offsets: OffsetsBuffer<O>,
        geom_index: usize,
    ) -> Self {
        Self {
            coords: Cow::Owned(coords),
            geom_offsets: Cow::Owned(geom_offsets),
            ring_offsets: Cow::Owned(ring_offsets),
            geom_index,
        }
    }
}

impl<'a, O: Offset> GeometryScalarTrait<'a> for MultiLineString<'a, O> {
    type ScalarGeo = geo::MultiLineString;

    fn to_geo(&self) -> Self::ScalarGeo {
        self.into()
    }
}

impl<'a, O: Offset> MultiLineStringTrait<'a> for MultiLineString<'a, O> {
    type T = f64;
    type ItemType = LineString<'a, O>;
    type Iter = MultiLineStringIterator<'a, O>;

    fn lines(&'a self) -> Self::Iter {
        MultiLineStringIterator::new(self)
    }

    fn num_lines(&self) -> usize {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        end - start
    }

    fn line(&self, i: usize) -> Option<Self::ItemType> {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        if i > (end - start) {
            return None;
        }

        Some(LineString::new(
            self.coords.clone(),
            self.ring_offsets.clone(),
            start + i,
        ))
    }
}

impl<'a, O: Offset> MultiLineStringTrait<'a> for &MultiLineString<'a, O> {
    type T = f64;
    type ItemType = LineString<'a, O>;
    type Iter = MultiLineStringIterator<'a, O>;

    fn lines(&'a self) -> Self::Iter {
        MultiLineStringIterator::new(self)
    }

    fn num_lines(&self) -> usize {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        end - start
    }

    fn line(&self, i: usize) -> Option<Self::ItemType> {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        if i > (end - start) {
            return None;
        }

        Some(LineString::new(
            self.coords.clone(),
            self.ring_offsets.clone(),
            start + i,
        ))
    }
}

impl<O: Offset> From<MultiLineString<'_, O>> for geo::MultiLineString {
    fn from(value: MultiLineString<'_, O>) -> Self {
        (&value).into()
    }
}

impl<O: Offset> From<&MultiLineString<'_, O>> for geo::MultiLineString {
    fn from(value: &MultiLineString<'_, O>) -> Self {
        // Start and end indices into the ring_offsets buffer
        let (start_geom_idx, end_geom_idx) = value.geom_offsets.start_end(value.geom_index);

        let mut line_strings: Vec<geo::LineString> =
            Vec::with_capacity(end_geom_idx - start_geom_idx);

        for ring_idx in start_geom_idx..end_geom_idx {
            let (start_coord_idx, end_coord_idx) = value.ring_offsets.start_end(ring_idx);
            let mut ring: Vec<geo::Coord> = Vec::with_capacity(end_coord_idx - start_coord_idx);
            for coord_idx in start_coord_idx..end_coord_idx {
                ring.push(value.coords.value(coord_idx).into())
            }
            line_strings.push(ring.into());
        }

        geo::MultiLineString::new(line_strings)
    }
}

impl<O: Offset> From<MultiLineString<'_, O>> for geo::Geometry {
    fn from(value: MultiLineString<'_, O>) -> Self {
        geo::Geometry::MultiLineString(value.into())
    }
}

impl<O: Offset> RTreeObject for MultiLineString<'_, O> {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let (lower, upper) = bounding_rect_multilinestring(self);
        AABB::from_corners(lower, upper)
    }
}

impl<O: Offset> PartialEq for MultiLineString<'_, O> {
    fn eq(&self, other: &Self) -> bool {
        if self.num_lines() != other.num_lines() {
            return false;
        }

        for i in 0..self.num_lines() {
            if self.line(i) != other.line(i) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use crate::array::MultiLineStringArray;
    use crate::test::multilinestring::{ml0, ml1};
    use crate::GeometryArrayTrait;

    /// Test Eq where the current index is true but another index is false
    #[test]
    fn test_eq_other_index_false() {
        let arr1: MultiLineStringArray<i32> = vec![ml0(), ml1()].into();
        let arr2: MultiLineStringArray<i32> = vec![ml0(), ml0()].into();

        assert_eq!(arr1.value(0), arr2.value(0));
        assert_ne!(arr1.value(1), arr2.value(1));
    }
}
