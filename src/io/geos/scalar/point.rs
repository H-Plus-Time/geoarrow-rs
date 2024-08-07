use crate::error::{GeoArrowError, Result};
use crate::geo_traits::{CoordTrait, PointTrait};
use crate::scalar::Point;
use geos::{CoordDimensions, CoordSeq, Geom, GeometryTypes};

impl<const D: usize> TryFrom<Point<'_, D>> for geos::Geometry {
    type Error = geos::Error;

    fn try_from(value: Point<'_, D>) -> std::result::Result<geos::Geometry, geos::Error> {
        geos::Geometry::try_from(&value)
    }
}

impl<'a, const D: usize> TryFrom<&'a Point<'_, D>> for geos::Geometry {
    type Error = geos::Error;

    fn try_from(value: &'a Point<'_, D>) -> std::result::Result<geos::Geometry, geos::Error> {
        let mut coord_seq = CoordSeq::new(1, CoordDimensions::TwoD)?;
        coord_seq.set_x(0, PointTrait::x(&value))?;
        coord_seq.set_y(0, PointTrait::y(&value))?;

        geos::Geometry::create_point(coord_seq)
    }
}

#[derive(Clone)]
pub struct GEOSPoint(geos::Geometry);

impl GEOSPoint {
    pub fn new_unchecked(geom: geos::Geometry) -> Self {
        Self(geom)
    }

    pub fn try_new(geom: geos::Geometry) -> Result<Self> {
        if matches!(geom.geometry_type(), GeometryTypes::Point) {
            Ok(Self(geom))
        } else {
            Err(GeoArrowError::General(
                "Geometry type must be point".to_string(),
            ))
        }
    }
}

impl PointTrait for GEOSPoint {
    type T = f64;

    fn dim(&self) -> usize {
        self.0.get_num_dimensions().unwrap()
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        match n {
            0 => self.0.get_x().unwrap(),
            1 => self.0.get_y().unwrap(),
            2 => self.0.get_z().unwrap(),
            _ => panic!(),
        }
    }

    fn x(&self) -> Self::T {
        self.0.get_x().unwrap()
    }

    fn y(&self) -> Self::T {
        self.0.get_y().unwrap()
    }
}

impl PointTrait for &GEOSPoint {
    type T = f64;

    fn dim(&self) -> usize {
        self.0.get_num_dimensions().unwrap()
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        match n {
            0 => self.0.get_x().unwrap(),
            1 => self.0.get_y().unwrap(),
            2 => self.0.get_z().unwrap(),
            _ => panic!(),
        }
    }

    fn x(&self) -> Self::T {
        self.0.get_x().unwrap()
    }

    fn y(&self) -> Self::T {
        self.0.get_y().unwrap()
    }
}

impl CoordTrait for GEOSPoint {
    type T = f64;

    fn dim(&self) -> usize {
        self.0.get_num_dimensions().unwrap()
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        match n {
            0 => self.0.get_x().unwrap(),
            1 => self.0.get_y().unwrap(),
            2 => self.0.get_z().unwrap(),
            _ => panic!(),
        }
    }

    fn x(&self) -> Self::T {
        self.0.get_x().unwrap()
    }

    fn y(&self) -> Self::T {
        self.0.get_y().unwrap()
    }
}

impl CoordTrait for &GEOSPoint {
    type T = f64;

    fn dim(&self) -> usize {
        self.0.get_num_dimensions().unwrap()
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        match n {
            0 => self.0.get_x().unwrap(),
            1 => self.0.get_y().unwrap(),
            2 => self.0.get_z().unwrap(),
            _ => panic!(),
        }
    }

    fn x(&self) -> Self::T {
        self.0.get_x().unwrap()
    }

    fn y(&self) -> Self::T {
        self.0.get_y().unwrap()
    }
}

pub struct GEOSConstPoint<'a>(geos::ConstGeometry<'a>);

impl<'a> GEOSConstPoint<'a> {
    pub fn new_unchecked(geom: geos::ConstGeometry<'a>) -> Self {
        Self(geom)
    }

    pub fn try_new(geom: geos::ConstGeometry<'a>) -> Result<Self> {
        if matches!(geom.geometry_type(), GeometryTypes::Point) {
            Ok(Self(geom))
        } else {
            Err(GeoArrowError::General(
                "Geometry type must be point".to_string(),
            ))
        }
    }
}

impl<'a> PointTrait for GEOSConstPoint<'a> {
    type T = f64;

    fn dim(&self) -> usize {
        self.0.get_num_dimensions().unwrap()
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        match n {
            0 => self.0.get_x().unwrap(),
            1 => self.0.get_y().unwrap(),
            2 => self.0.get_z().unwrap(),
            _ => panic!(),
        }
    }

    fn x(&self) -> Self::T {
        self.0.get_x().unwrap()
    }

    fn y(&self) -> Self::T {
        self.0.get_y().unwrap()
    }
}

impl<'a> PointTrait for &GEOSConstPoint<'a> {
    type T = f64;

    fn dim(&self) -> usize {
        self.0.get_num_dimensions().unwrap()
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        match n {
            0 => self.0.get_x().unwrap(),
            1 => self.0.get_y().unwrap(),
            2 => self.0.get_z().unwrap(),
            _ => panic!(),
        }
    }

    fn x(&self) -> Self::T {
        self.0.get_x().unwrap()
    }

    fn y(&self) -> Self::T {
        self.0.get_y().unwrap()
    }
}

impl<'a> CoordTrait for GEOSConstPoint<'a> {
    type T = f64;

    fn dim(&self) -> usize {
        self.0.get_num_dimensions().unwrap()
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        match n {
            0 => self.0.get_x().unwrap(),
            1 => self.0.get_y().unwrap(),
            2 => self.0.get_z().unwrap(),
            _ => panic!(),
        }
    }

    fn x(&self) -> Self::T {
        self.0.get_x().unwrap()
    }

    fn y(&self) -> Self::T {
        self.0.get_y().unwrap()
    }
}

impl<'a> CoordTrait for &GEOSConstPoint<'a> {
    type T = f64;

    fn dim(&self) -> usize {
        self.0.get_num_dimensions().unwrap()
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        match n {
            0 => self.0.get_x().unwrap(),
            1 => self.0.get_y().unwrap(),
            2 => self.0.get_z().unwrap(),
            _ => panic!(),
        }
    }

    fn x(&self) -> Self::T {
        self.0.get_x().unwrap()
    }

    fn y(&self) -> Self::T {
        self.0.get_y().unwrap()
    }
}

impl Clone for GEOSConstPoint<'_> {
    fn clone(&self) -> Self {
        todo!()
    }
}
