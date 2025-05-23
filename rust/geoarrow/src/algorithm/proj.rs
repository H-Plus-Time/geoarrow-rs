//! Bindings to the [`proj`] crate for coordinate reprojection.

use crate::array::*;
use crate::error::Result;
use crate::trait_::ArrayAccessor;
use geoarrow_schema::Dimension;
use proj::{Proj, Transform};

/// Reproject an array using PROJ
///
/// Note: this will currently return a two-dimensional array
pub trait Reproject {
    fn reproject(&self, proj: &Proj) -> Result<Self>
    where
        Self: Sized;
}

impl Reproject for PointArray {
    fn reproject(&self, proj: &Proj) -> Result<Self> {
        let mut output_array = PointBuilder::with_capacity(Dimension::XY, self.len());

        for maybe_geom in self.iter_geo() {
            if let Some(mut geom) = maybe_geom {
                geom.transform(proj)?;
                output_array.push_point(Some(&geom));
            } else {
                output_array.push_null()
            }
        }

        Ok(output_array.into())
    }
}

macro_rules! iter_geo_impl {
    ($type:ty, $builder_type:ty, $push_func:ident) => {
        impl Reproject for $type {
            fn reproject(&self, proj: &Proj) -> Result<Self> {
                let mut output_array =
                    <$builder_type>::with_capacity(Dimension::XY, self.buffer_lengths());

                for maybe_geom in self.iter_geo() {
                    if let Some(mut geom) = maybe_geom {
                        geom.transform(proj)?;
                        output_array.$push_func(Some(&geom))?;
                    } else {
                        output_array.push_null()
                    }
                }

                Ok(output_array.into())
            }
        }
    };
}

iter_geo_impl!(LineStringArray, LineStringBuilder, push_line_string);
iter_geo_impl!(PolygonArray, PolygonBuilder, push_polygon);
iter_geo_impl!(MultiPointArray, MultiPointBuilder, push_multi_point);
iter_geo_impl!(
    MultiLineStringArray,
    MultiLineStringBuilder,
    push_multi_line_string
);
iter_geo_impl!(MultiPolygonArray, MultiPolygonBuilder, push_multi_polygon);

#[cfg(test)]
mod test {
    use crate::trait_::ArrayAccessor;
    use approx::assert_relative_eq;

    use super::*;
    use crate::test::point::{p0, p1, p2};

    #[test]
    fn point_round_trip() {
        let point_array: PointArray =
            (vec![Some(p0()), Some(p1()), Some(p2())], Dimension::XY).into();
        let proj = Proj::new_known_crs("EPSG:4326", "EPSG:3857", None).unwrap();

        // You can verify this with PROJ on the command line:
        // echo 1 0 | cs2cs EPSG:4326 EPSG:3857
        // 0.00	111325.14 0.00
        // Though note that cs2cs is using y/x for EPSG:4326
        let out = point_array.reproject(&proj).unwrap();
        assert_eq!(out.value_as_geo(0).x(), 0.0);
        assert_relative_eq!(out.value_as_geo(0).y(), 111325.1428663851);
        dbg!(out);
    }
}
