use std::sync::Arc;

use crate::array::*;
use crate::chunked_array::{ChunkedGeometryArray, ChunkedGeometryArrayTrait};
use crate::datatypes::{Dimension, GeoDataType};
use crate::error::{GeoArrowError, Result};
use crate::trait_::GeometryArrayAccessor;
use crate::GeometryArrayTrait;
use arrow_array::OffsetSizeTrait;
use geo::Simplify as _Simplify;

/// Simplifies a geometry.
///
/// The [Ramer–Douglas–Peucker
/// algorithm](https://en.wikipedia.org/wiki/Ramer–Douglas–Peucker_algorithm) simplifies a
/// linestring. Polygons are simplified by running the RDP algorithm on all their constituent
/// rings. This may result in invalid Polygons, and has no guarantee of preserving topology.
///
/// Multi* objects are simplified by simplifying all their constituent geometries individually.
///
/// An epsilon less than or equal to zero will return an unaltered version of the geometry.
pub trait Simplify {
    type Output;

    /// Returns the simplified representation of a geometry, using the [Ramer–Douglas–Peucker](https://en.wikipedia.org/wiki/Ramer–Douglas–Peucker_algorithm) algorithm
    ///
    /// # Examples
    ///
    /// ```
    /// use geoarrow::algorithm::geo::Simplify;
    /// use geoarrow::array::LineStringArray;
    /// use geoarrow::trait_::GeometryArrayAccessor;
    /// use geo::line_string;
    ///
    /// let line_string = line_string![
    ///     (x: 0.0, y: 0.0),
    ///     (x: 5.0, y: 4.0),
    ///     (x: 11.0, y: 5.5),
    ///     (x: 17.3, y: 3.2),
    ///     (x: 27.8, y: 0.1),
    /// ];
    /// let line_string_array: LineStringArray<i32, 2> = vec![line_string].as_slice().into();
    ///
    /// let simplified_array = line_string_array.simplify(&1.0);
    ///
    /// let expected = line_string![
    ///     (x: 0.0, y: 0.0),
    ///     (x: 5.0, y: 4.0),
    ///     (x: 11.0, y: 5.5),
    ///     (x: 27.8, y: 0.1),
    /// ];
    ///
    /// assert_eq!(expected, simplified_array.value_as_geo(0))
    /// ```
    fn simplify(&self, epsilon: &f64) -> Self::Output;
}

// Note: this can't (easily) be parameterized in the macro because PointArray is not generic over O
impl Simplify for PointArray<2> {
    type Output = Self;

    fn simplify(&self, _epsilon: &f64) -> Self {
        self.clone()
    }
}

/// Implementation that returns the identity
macro_rules! identity_impl {
    ($type:ty) => {
        impl<O: OffsetSizeTrait> Simplify for $type {
            type Output = Self;

            fn simplify(&self, _epsilon: &f64) -> Self {
                self.clone()
            }
        }
    };
}

identity_impl!(MultiPointArray<O, 2>);

/// Implementation that iterates over geo objects
macro_rules! iter_geo_impl {
    ($type:ty, $geo_type:ty) => {
        impl<O: OffsetSizeTrait> Simplify for $type {
            type Output = Self;

            fn simplify(&self, epsilon: &f64) -> Self {
                let output_geoms: Vec<Option<$geo_type>> = self
                    .iter_geo()
                    .map(|maybe_g| maybe_g.map(|geom| geom.simplify(epsilon)))
                    .collect();

                output_geoms.into()
            }
        }
    };
}

iter_geo_impl!(LineStringArray<O, 2>, geo::LineString);
iter_geo_impl!(PolygonArray<O, 2>, geo::Polygon);
iter_geo_impl!(MultiLineStringArray<O, 2>, geo::MultiLineString);
iter_geo_impl!(MultiPolygonArray<O, 2>, geo::MultiPolygon);
// iter_geo_impl!(MixedGeometryArray<O, 2>, geo::Geometry);
// iter_geo_impl!(GeometryCollectionArray<O, 2>, geo::GeometryCollection);

impl Simplify for &dyn GeometryArrayTrait {
    type Output = Result<Arc<dyn GeometryArrayTrait>>;

    fn simplify(&self, epsilon: &f64) -> Self::Output {
        let result: Arc<dyn GeometryArrayTrait> = match self.data_type() {
            GeoDataType::Point(_, Dimension::XY) => Arc::new(self.as_point_2d().simplify(epsilon)),
            GeoDataType::LineString(_, Dimension::XY) => {
                Arc::new(self.as_line_string_2d().simplify(epsilon))
            }
            GeoDataType::LargeLineString(_, Dimension::XY) => {
                Arc::new(self.as_large_line_string_2d().simplify(epsilon))
            }
            GeoDataType::Polygon(_, Dimension::XY) => {
                Arc::new(self.as_polygon_2d().simplify(epsilon))
            }
            GeoDataType::LargePolygon(_, Dimension::XY) => {
                Arc::new(self.as_large_polygon_2d().simplify(epsilon))
            }
            GeoDataType::MultiPoint(_, Dimension::XY) => {
                Arc::new(self.as_multi_point_2d().simplify(epsilon))
            }
            GeoDataType::LargeMultiPoint(_, Dimension::XY) => {
                Arc::new(self.as_large_multi_point_2d().simplify(epsilon))
            }
            GeoDataType::MultiLineString(_, Dimension::XY) => {
                Arc::new(self.as_multi_line_string_2d().simplify(epsilon))
            }
            GeoDataType::LargeMultiLineString(_, Dimension::XY) => {
                Arc::new(self.as_large_multi_line_string_2d().simplify(epsilon))
            }
            GeoDataType::MultiPolygon(_, Dimension::XY) => {
                Arc::new(self.as_multi_polygon_2d().simplify(epsilon))
            }
            GeoDataType::LargeMultiPolygon(_, Dimension::XY) => {
                Arc::new(self.as_large_multi_polygon_2d().simplify(epsilon))
            }
            // GeoDataType::Mixed(_, Dimension::XY) => self.as_mixed_2d().simplify(epsilon),
            // GeoDataType::LargeMixed(_, Dimension::XY) => self.as_large_mixed_2d().simplify(),
            // GeoDataType::GeometryCollection(_, Dimension::XY) => self.as_geometry_collection_2d().simplify(),
            // GeoDataType::LargeGeometryCollection(_, Dimension::XY) => {
            //     self.as_large_geometry_collection_2d().simplify()
            // }
            _ => return Err(GeoArrowError::IncorrectType("".into())),
        };
        Ok(result)
    }
}

impl Simplify for ChunkedGeometryArray<PointArray<2>> {
    type Output = Self;

    fn simplify(&self, epsilon: &f64) -> Self::Output {
        self.map(|chunk| chunk.simplify(epsilon))
            .try_into()
            .unwrap()
    }
}

/// Implementation that iterates over chunks
macro_rules! chunked_impl {
    ($type:ty) => {
        impl<O: OffsetSizeTrait> Simplify for $type {
            type Output = Self;

            fn simplify(&self, epsilon: &f64) -> Self {
                self.map(|chunk| chunk.simplify(epsilon))
                    .try_into()
                    .unwrap()
            }
        }
    };
}

chunked_impl!(ChunkedGeometryArray<LineStringArray<O, 2>>);
chunked_impl!(ChunkedGeometryArray<PolygonArray<O, 2>>);
chunked_impl!(ChunkedGeometryArray<MultiPointArray<O, 2>>);
chunked_impl!(ChunkedGeometryArray<MultiLineStringArray<O, 2>>);
chunked_impl!(ChunkedGeometryArray<MultiPolygonArray<O, 2>>);

impl Simplify for &dyn ChunkedGeometryArrayTrait {
    type Output = Result<Arc<dyn ChunkedGeometryArrayTrait>>;

    fn simplify(&self, epsilon: &f64) -> Self::Output {
        let result: Arc<dyn ChunkedGeometryArrayTrait> = match self.data_type() {
            GeoDataType::Point(_, Dimension::XY) => Arc::new(self.as_point_2d().simplify(epsilon)),
            GeoDataType::LineString(_, Dimension::XY) => {
                Arc::new(self.as_line_string_2d().simplify(epsilon))
            }
            GeoDataType::LargeLineString(_, Dimension::XY) => {
                Arc::new(self.as_large_line_string_2d().simplify(epsilon))
            }
            GeoDataType::Polygon(_, Dimension::XY) => {
                Arc::new(self.as_polygon_2d().simplify(epsilon))
            }
            GeoDataType::LargePolygon(_, Dimension::XY) => {
                Arc::new(self.as_large_polygon_2d().simplify(epsilon))
            }
            GeoDataType::MultiPoint(_, Dimension::XY) => {
                Arc::new(self.as_multi_point_2d().simplify(epsilon))
            }
            GeoDataType::LargeMultiPoint(_, Dimension::XY) => {
                Arc::new(self.as_large_multi_point_2d().simplify(epsilon))
            }
            GeoDataType::MultiLineString(_, Dimension::XY) => {
                Arc::new(self.as_multi_line_string_2d().simplify(epsilon))
            }
            GeoDataType::LargeMultiLineString(_, Dimension::XY) => {
                Arc::new(self.as_large_multi_line_string_2d().simplify(epsilon))
            }
            GeoDataType::MultiPolygon(_, Dimension::XY) => {
                Arc::new(self.as_multi_polygon_2d().simplify(epsilon))
            }
            GeoDataType::LargeMultiPolygon(_, Dimension::XY) => {
                Arc::new(self.as_large_multi_polygon_2d().simplify(epsilon))
            }
            // GeoDataType::Mixed(_, Dimension::XY) => self.as_mixed_2d().simplify(epsilon),
            // GeoDataType::LargeMixed(_, Dimension::XY) => self.as_large_mixed_2d().simplify(),
            // GeoDataType::GeometryCollection(_, Dimension::XY) => self.as_geometry_collection_2d().simplify(),
            // GeoDataType::LargeGeometryCollection(_, Dimension::XY) => {
            //     self.as_large_geometry_collection_2d().simplify()
            // }
            _ => return Err(GeoArrowError::IncorrectType("".into())),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array::{LineStringArray, PolygonArray};
    use crate::trait_::GeometryArrayAccessor;
    use geo::{line_string, polygon};

    #[test]
    fn rdp_test() {
        let input_geom = line_string![
            (x: 0.0, y: 0.0 ),
            (x: 5.0, y: 4.0 ),
            (x: 11.0, y: 5.5 ),
            (x: 17.3, y: 3.2 ),
            (x: 27.8, y: 0.1 ),
        ];
        let input_array: LineStringArray<i64, 2> = vec![input_geom].as_slice().into();
        let result_array = input_array.simplify(&1.0);

        let expected = line_string![
            ( x: 0.0, y: 0.0 ),
            ( x: 5.0, y: 4.0 ),
            ( x: 11.0, y: 5.5 ),
            ( x: 27.8, y: 0.1 ),
        ];

        assert_eq!(expected, result_array.get_as_geo(0).unwrap());
    }

    #[test]
    fn polygon() {
        let input_geom = polygon![
            (x: 0., y: 0.),
            (x: 0., y: 10.),
            (x: 5., y: 11.),
            (x: 10., y: 10.),
            (x: 10., y: 0.),
            (x: 0., y: 0.),
        ];
        let input_array: PolygonArray<i64, 2> = vec![input_geom].as_slice().into();
        let result_array = input_array.simplify(&2.0);

        let expected = polygon![
            (x: 0., y: 0.),
            (x: 0., y: 10.),
            (x: 10., y: 10.),
            (x: 10., y: 0.),
            (x: 0., y: 0.),
        ];

        assert_eq!(expected, result_array.get_as_geo(0).unwrap());
    }
}
