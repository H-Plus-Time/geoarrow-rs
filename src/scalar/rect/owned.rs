use crate::algorithm::native::eq::rect_eq;
use crate::array::{RectArray, SeparatedCoordBuffer};
use crate::geo_traits::RectTrait;
use crate::scalar::Rect;

#[derive(Clone, Debug)]
pub struct OwnedRect<const D: usize> {
    lower: SeparatedCoordBuffer<D>,
    upper: SeparatedCoordBuffer<D>,
    geom_index: usize,
}

impl<const D: usize> OwnedRect<D> {
    pub fn new(
        lower: SeparatedCoordBuffer<D>,
        upper: SeparatedCoordBuffer<D>,
        geom_index: usize,
    ) -> Self {
        Self {
            lower,
            upper,
            geom_index,
        }
    }
}

impl<'a, const D: usize> From<&'a OwnedRect<D>> for Rect<'a, D> {
    fn from(value: &'a OwnedRect<D>) -> Self {
        Self::new(&value.lower, &value.upper, value.geom_index)
    }
}

impl<'a, const D: usize> From<Rect<'a, D>> for OwnedRect<D> {
    fn from(value: Rect<'a, D>) -> Self {
        let (lower, upper, geom_index) = value.into_owned_inner();
        Self::new(lower, upper, geom_index)
    }
}

impl<const D: usize> From<OwnedRect<D>> for RectArray<D> {
    fn from(value: OwnedRect<D>) -> Self {
        Self::new(value.lower, value.upper, None, Default::default())
    }
}

impl<const D: usize> RectTrait for OwnedRect<D> {
    type T = f64;
    type ItemType<'b> = [Self::T; D] where Self: 'b;

    fn dim(&self) -> usize {
        D
    }

    fn lower(&self) -> Self::ItemType<'_> {
        Rect::from(self).lower()
    }

    fn upper(&self) -> Self::ItemType<'_> {
        Rect::from(self).upper()
    }
}

impl<G: RectTrait<T = f64>> PartialEq<G> for OwnedRect<2> {
    fn eq(&self, other: &G) -> bool {
        rect_eq(self, other)
    }
}
