use crate::NativeArray;
use crate::algorithm::native::Unary;
use crate::array::*;
use crate::chunked_array::{ChunkedArray, ChunkedGeometryArray};
use crate::datatypes::NativeType;
use crate::error::Result;
use crate::trait_::NativeScalar;
use arrow_array::BooleanArray;
use geos::Geom;

/// Checks if the geometry is valid
pub trait IsValid {
    type Output;

    fn is_valid(&self) -> Self::Output;
}

macro_rules! iter_geos_impl {
    ($type:ty) => {
        impl IsValid for $type {
            type Output = Result<BooleanArray>;

            fn is_valid(&self) -> Self::Output {
                Ok(self
                    .try_unary_boolean(|geom| Ok::<_, geos::Error>(geom.to_geos()?.is_valid()))?)
            }
        }
    };
}

iter_geos_impl!(PointArray);
iter_geos_impl!(LineStringArray);
iter_geos_impl!(MultiPointArray);
iter_geos_impl!(MultiLineStringArray);
iter_geos_impl!(PolygonArray);
iter_geos_impl!(MultiPolygonArray);
iter_geos_impl!(MixedGeometryArray);
iter_geos_impl!(GeometryCollectionArray);
iter_geos_impl!(RectArray);
iter_geos_impl!(GeometryArray);

impl IsValid for &dyn NativeArray {
    type Output = Result<BooleanArray>;

    fn is_valid(&self) -> Self::Output {
        use NativeType::*;

        match self.data_type() {
            Point(_) => IsValid::is_valid(self.as_point()),
            LineString(_) => IsValid::is_valid(self.as_line_string()),
            Polygon(_) => IsValid::is_valid(self.as_polygon()),
            MultiPoint(_) => IsValid::is_valid(self.as_multi_point()),
            MultiLineString(_) => IsValid::is_valid(self.as_multi_line_string()),
            MultiPolygon(_) => IsValid::is_valid(self.as_multi_polygon()),
            GeometryCollection(_) => IsValid::is_valid(self.as_geometry_collection()),
            Rect(_) => IsValid::is_valid(self.as_rect()),
            Geometry(_) => IsValid::is_valid(self.as_geometry()),
        }
    }
}

impl<G: NativeArray> IsValid for ChunkedGeometryArray<G> {
    type Output = Result<ChunkedArray<BooleanArray>>;

    fn is_valid(&self) -> Self::Output {
        let mut output_chunks = Vec::with_capacity(self.chunks.len());
        for chunk in self.chunks.iter() {
            output_chunks.push(IsValid::is_valid(&chunk.as_ref())?);
        }

        Ok(ChunkedArray::new(output_chunks))
    }
}
