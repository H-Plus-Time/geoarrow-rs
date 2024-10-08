use std::sync::Arc;

use super::MultiPointBuilder;
use crate::algorithm::native::eq::offset_buffer_eq;
use crate::array::metadata::ArrayMetadata;
use crate::array::multipoint::MultiPointCapacity;
use crate::array::util::{
    offsets_buffer_i32_to_i64, offsets_buffer_i64_to_i32, offsets_buffer_to_i32,
    offsets_buffer_to_i64, OffsetBufferUtils,
};
use crate::array::{
    CoordBuffer, CoordType, GeometryCollectionArray, LineStringArray, MixedGeometryArray,
    PointArray, WKBArray,
};
use crate::datatypes::GeoDataType;
use crate::error::{GeoArrowError, Result};
use crate::geo_traits::MultiPointTrait;
use crate::scalar::MultiPoint;
use crate::trait_::{GeometryArraySelfMethods, IntoArrow, NativeArrayAccessor};
use crate::util::{owned_slice_offsets, owned_slice_validity};
use crate::NativeArray;
use arrow_array::{Array, GenericListArray, LargeListArray, ListArray, OffsetSizeTrait};
use arrow_buffer::{NullBuffer, OffsetBuffer};
use arrow_schema::{DataType, Field};

/// An immutable array of MultiPoint geometries using GeoArrow's in-memory representation.
///
/// This is semantically equivalent to `Vec<Option<MultiPoint>>` due to the internal validity
/// bitmap.
#[derive(Debug, Clone)]
pub struct MultiPointArray<O: OffsetSizeTrait, const D: usize> {
    // Always GeoDataType::MultiPoint or GeoDataType::LargeMultiPoint
    data_type: GeoDataType,

    pub(crate) metadata: Arc<ArrayMetadata>,

    pub(crate) coords: CoordBuffer<D>,

    /// Offsets into the coordinate array where each geometry starts
    pub(crate) geom_offsets: OffsetBuffer<O>,

    /// Validity bitmap
    pub(crate) validity: Option<NullBuffer>,
}

pub(super) fn check<O: OffsetSizeTrait, const D: usize>(
    coords: &CoordBuffer<D>,
    validity_len: Option<usize>,
    geom_offsets: &OffsetBuffer<O>,
) -> Result<()> {
    if validity_len.map_or(false, |len| len != geom_offsets.len_proxy()) {
        return Err(GeoArrowError::General(
            "validity mask length must match the number of values".to_string(),
        ));
    }

    if geom_offsets.last().to_usize().unwrap() != coords.len() {
        return Err(GeoArrowError::General(
            "largest geometry offset must match coords length".to_string(),
        ));
    }

    Ok(())
}

impl<O: OffsetSizeTrait, const D: usize> MultiPointArray<O, D> {
    /// Create a new MultiPointArray from parts
    ///
    /// # Implementation
    ///
    /// This function is `O(1)`.
    ///
    /// # Panics
    ///
    /// - if the validity is not `None` and its length is different from the number of geometries
    /// - if the largest geometry offset does not match the number of coordinates
    pub fn new(
        coords: CoordBuffer<D>,
        geom_offsets: OffsetBuffer<O>,
        validity: Option<NullBuffer>,
        metadata: Arc<ArrayMetadata>,
    ) -> Self {
        Self::try_new(coords, geom_offsets, validity, metadata).unwrap()
    }

    /// Create a new MultiPointArray from parts
    ///
    /// # Implementation
    ///
    /// This function is `O(1)`.
    ///
    /// # Errors
    ///
    /// - if the validity is not `None` and its length is different from the number of geometries
    /// - if the geometry offsets do not match the number of coordinates
    pub fn try_new(
        coords: CoordBuffer<D>,
        geom_offsets: OffsetBuffer<O>,
        validity: Option<NullBuffer>,
        metadata: Arc<ArrayMetadata>,
    ) -> Result<Self> {
        check(&coords, validity.as_ref().map(|v| v.len()), &geom_offsets)?;

        let coord_type = coords.coord_type();
        let data_type = match O::IS_LARGE {
            true => GeoDataType::LargeMultiPoint(coord_type, D.try_into()?),
            false => GeoDataType::MultiPoint(coord_type, D.try_into()?),
        };

        Ok(Self {
            data_type,
            coords,
            geom_offsets,
            validity,
            metadata,
        })
    }

    fn vertices_field(&self) -> Arc<Field> {
        Field::new("points", self.coords.storage_type(), false).into()
    }

    pub fn coords(&self) -> &CoordBuffer<D> {
        &self.coords
    }

    pub fn into_inner(self) -> (CoordBuffer<D>, OffsetBuffer<O>, Option<NullBuffer>) {
        (self.coords, self.geom_offsets, self.validity)
    }

    pub fn geom_offsets(&self) -> &OffsetBuffer<O> {
        &self.geom_offsets
    }

    /// The lengths of each buffer contained in this array.
    pub fn buffer_lengths(&self) -> MultiPointCapacity {
        MultiPointCapacity::new(self.geom_offsets.last().to_usize().unwrap(), self.len())
    }

    /// The number of bytes occupied by this array.
    pub fn num_bytes(&self) -> usize {
        let validity_len = self.nulls().map(|v| v.buffer().len()).unwrap_or(0);
        validity_len + self.buffer_lengths().num_bytes::<O>()
    }

    /// Slices this [`MultiPointArray`] in place.
    /// # Implementation
    /// This operation is `O(1)` as it amounts to increase two ref counts.
    /// # Examples
    /// ```
    /// use arrow::array::PrimitiveArray;
    /// use arrow_array::types::Int32Type;
    ///
    /// let array: PrimitiveArray<Int32Type> = PrimitiveArray::from(vec![1, 2, 3]);
    /// assert_eq!(format!("{:?}", array), "PrimitiveArray<Int32>\n[\n  1,\n  2,\n  3,\n]");
    /// let sliced = array.slice(1, 1);
    /// assert_eq!(format!("{:?}", sliced), "PrimitiveArray<Int32>\n[\n  2,\n]");
    /// // note: `sliced` and `array` share the same memory region.
    /// ```
    /// # Panic
    /// This function panics iff `offset + length > self.len()`.
    #[inline]
    pub fn slice(&self, offset: usize, length: usize) -> Self {
        assert!(
            offset + length <= self.len(),
            "offset + length may not exceed length of array"
        );
        // Note: we **only** slice the geom_offsets and not any actual data. Otherwise the offsets
        // would be in the wrong location.
        Self {
            data_type: self.data_type,
            coords: self.coords.clone(),
            geom_offsets: self.geom_offsets.slice(offset, length),
            validity: self.validity.as_ref().map(|v| v.slice(offset, length)),
            metadata: self.metadata(),
        }
    }

    pub fn owned_slice(&self, offset: usize, length: usize) -> Self {
        assert!(
            offset + length <= self.len(),
            "offset + length may not exceed length of array"
        );
        assert!(length >= 1, "length must be at least 1");

        // Find the start and end of the coord buffer
        let (start_coord_idx, _) = self.geom_offsets.start_end(offset);
        let (_, end_coord_idx) = self.geom_offsets.start_end(offset + length - 1);

        let geom_offsets = owned_slice_offsets(&self.geom_offsets, offset, length);

        let coords = self
            .coords
            .owned_slice(start_coord_idx, end_coord_idx - start_coord_idx);

        let validity = owned_slice_validity(self.nulls(), offset, length);

        Self::new(coords, geom_offsets, validity, self.metadata())
    }

    pub fn to_coord_type(&self, coord_type: CoordType) -> Self {
        self.clone().into_coord_type(coord_type)
    }

    pub fn into_coord_type(self, coord_type: CoordType) -> Self {
        Self::new(
            self.coords.into_coord_type(coord_type),
            self.geom_offsets,
            self.validity,
            self.metadata,
        )
    }

    pub fn to_small_offsets(&self) -> Result<MultiPointArray<i32, D>> {
        Ok(MultiPointArray::new(
            self.coords.clone(),
            offsets_buffer_to_i32(&self.geom_offsets)?,
            self.validity.clone(),
            self.metadata.clone(),
        ))
    }

    pub fn to_large_offsets(&self) -> MultiPointArray<i64, D> {
        MultiPointArray::new(
            self.coords.clone(),
            offsets_buffer_to_i64(&self.geom_offsets),
            self.validity.clone(),
            self.metadata.clone(),
        )
    }
}

impl<O: OffsetSizeTrait, const D: usize> NativeArray for MultiPointArray<O, D> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn data_type(&self) -> GeoDataType {
        self.data_type
    }

    fn storage_type(&self) -> DataType {
        self.data_type.to_data_type()
    }

    fn extension_field(&self) -> Arc<Field> {
        self.data_type
            .to_field_with_metadata("geometry", true, &self.metadata)
            .into()
    }

    fn extension_name(&self) -> &str {
        self.data_type.extension_name()
    }

    fn into_array_ref(self) -> Arc<dyn Array> {
        Arc::new(self.into_arrow())
    }

    fn to_array_ref(&self) -> arrow_array::ArrayRef {
        self.clone().into_array_ref()
    }

    fn coord_type(&self) -> CoordType {
        self.coords.coord_type()
    }

    fn to_coord_type(&self, coord_type: CoordType) -> Arc<dyn NativeArray> {
        Arc::new(self.clone().into_coord_type(coord_type))
    }

    fn metadata(&self) -> Arc<ArrayMetadata> {
        self.metadata.clone()
    }

    fn with_metadata(&self, metadata: Arc<ArrayMetadata>) -> crate::trait_::NativeArrayRef {
        let mut arr = self.clone();
        arr.metadata = metadata;
        Arc::new(arr)
    }

    /// Returns the number of geometries in this array
    #[inline]
    fn len(&self) -> usize {
        self.geom_offsets.len_proxy()
    }

    /// Returns the optional validity.
    #[inline]
    fn nulls(&self) -> Option<&NullBuffer> {
        self.validity.as_ref()
    }

    fn as_ref(&self) -> &dyn NativeArray {
        self
    }

    fn slice(&self, offset: usize, length: usize) -> Arc<dyn NativeArray> {
        Arc::new(self.slice(offset, length))
    }

    fn owned_slice(&self, offset: usize, length: usize) -> Arc<dyn NativeArray> {
        Arc::new(self.owned_slice(offset, length))
    }
}

impl<O: OffsetSizeTrait, const D: usize> GeometryArraySelfMethods<D> for MultiPointArray<O, D> {
    fn with_coords(self, coords: CoordBuffer<D>) -> Self {
        assert_eq!(coords.len(), self.coords.len());
        Self::new(coords, self.geom_offsets, self.validity, self.metadata)
    }

    fn into_coord_type(self, coord_type: CoordType) -> Self {
        Self::new(
            self.coords.into_coord_type(coord_type),
            self.geom_offsets,
            self.validity,
            self.metadata,
        )
    }
}

// Implement geometry accessors
impl<'a, O: OffsetSizeTrait, const D: usize> NativeArrayAccessor<'a> for MultiPointArray<O, D> {
    type Item = MultiPoint<'a, O, D>;
    type ItemGeo = geo::MultiPoint;

    unsafe fn value_unchecked(&'a self, index: usize) -> Self::Item {
        MultiPoint::new(&self.coords, &self.geom_offsets, index)
    }
}

impl<O: OffsetSizeTrait, const D: usize> IntoArrow for MultiPointArray<O, D> {
    type ArrowArray = GenericListArray<O>;

    fn into_arrow(self) -> Self::ArrowArray {
        let vertices_field = self.vertices_field();
        let validity = self.validity;
        let coord_array = self.coords.into_arrow();
        GenericListArray::new(vertices_field, self.geom_offsets, coord_array, validity)
    }
}

impl<O: OffsetSizeTrait, const D: usize> TryFrom<&GenericListArray<O>> for MultiPointArray<O, D> {
    type Error = GeoArrowError;

    fn try_from(value: &GenericListArray<O>) -> Result<Self> {
        let coords: CoordBuffer<D> = value.values().as_ref().try_into()?;
        let geom_offsets = value.offsets();
        let validity = value.nulls();

        Ok(Self::new(
            coords,
            geom_offsets.clone(),
            validity.cloned(),
            Default::default(),
        ))
    }
}

impl<const D: usize> TryFrom<&dyn Array> for MultiPointArray<i32, D> {
    type Error = GeoArrowError;

    fn try_from(value: &dyn Array) -> Result<Self> {
        match value.data_type() {
            DataType::List(_) => {
                let downcasted = value.as_any().downcast_ref::<ListArray>().unwrap();
                downcasted.try_into()
            }
            DataType::LargeList(_) => {
                let downcasted = value.as_any().downcast_ref::<LargeListArray>().unwrap();
                let geom_array: MultiPointArray<i64, D> = downcasted.try_into()?;
                geom_array.try_into()
            }
            _ => Err(GeoArrowError::General(format!(
                "Unexpected type: {:?}",
                value.data_type()
            ))),
        }
    }
}

impl<const D: usize> TryFrom<&dyn Array> for MultiPointArray<i64, D> {
    type Error = GeoArrowError;

    fn try_from(value: &dyn Array) -> Result<Self> {
        match value.data_type() {
            DataType::List(_) => {
                let downcasted = value.as_any().downcast_ref::<ListArray>().unwrap();
                let geom_array: MultiPointArray<i32, D> = downcasted.try_into()?;
                Ok(geom_array.into())
            }
            DataType::LargeList(_) => {
                let downcasted = value.as_any().downcast_ref::<LargeListArray>().unwrap();
                downcasted.try_into()
            }
            _ => Err(GeoArrowError::General(format!(
                "Unexpected type: {:?}",
                value.data_type()
            ))),
        }
    }
}

impl<const D: usize> TryFrom<(&dyn Array, &Field)> for MultiPointArray<i32, D> {
    type Error = GeoArrowError;

    fn try_from((arr, field): (&dyn Array, &Field)) -> Result<Self> {
        let mut arr: Self = arr.try_into()?;
        arr.metadata = Arc::new(ArrayMetadata::try_from(field)?);
        Ok(arr)
    }
}

impl<const D: usize> TryFrom<(&dyn Array, &Field)> for MultiPointArray<i64, D> {
    type Error = GeoArrowError;

    fn try_from((arr, field): (&dyn Array, &Field)) -> Result<Self> {
        let mut arr: Self = arr.try_into()?;
        arr.metadata = Arc::new(ArrayMetadata::try_from(field)?);
        Ok(arr)
    }
}

impl<O: OffsetSizeTrait, G: MultiPointTrait<T = f64>, const D: usize> From<Vec<Option<G>>>
    for MultiPointArray<O, D>
{
    fn from(other: Vec<Option<G>>) -> Self {
        let mut_arr: MultiPointBuilder<O, D> = other.into();
        mut_arr.into()
    }
}

impl<O: OffsetSizeTrait, G: MultiPointTrait<T = f64>, const D: usize> From<&[G]>
    for MultiPointArray<O, D>
{
    fn from(other: &[G]) -> Self {
        let mut_arr: MultiPointBuilder<O, D> = other.into();
        mut_arr.into()
    }
}

impl<O: OffsetSizeTrait, const D: usize> TryFrom<WKBArray<O>> for MultiPointArray<O, D> {
    type Error = GeoArrowError;

    fn try_from(value: WKBArray<O>) -> Result<Self> {
        let mut_arr: MultiPointBuilder<O, D> = value.try_into()?;
        Ok(mut_arr.into())
    }
}

/// LineString and MultiPoint have the same layout, so enable conversions between the two to change
/// the semantic type
impl<O: OffsetSizeTrait, const D: usize> From<MultiPointArray<O, D>> for LineStringArray<O, D> {
    fn from(value: MultiPointArray<O, D>) -> Self {
        Self::new(
            value.coords,
            value.geom_offsets,
            value.validity,
            value.metadata,
        )
    }
}

impl<O: OffsetSizeTrait, const D: usize> From<PointArray<D>> for MultiPointArray<O, D> {
    fn from(value: PointArray<D>) -> Self {
        let coords = value.coords;
        let geom_offsets = OffsetBuffer::from_lengths(vec![1; coords.len()]);
        let validity = value.validity;
        Self::new(coords, geom_offsets, validity, value.metadata)
    }
}

impl<const D: usize> From<MultiPointArray<i32, D>> for MultiPointArray<i64, D> {
    fn from(value: MultiPointArray<i32, D>) -> Self {
        Self::new(
            value.coords,
            offsets_buffer_i32_to_i64(&value.geom_offsets),
            value.validity,
            value.metadata,
        )
    }
}

impl<const D: usize> TryFrom<MultiPointArray<i64, D>> for MultiPointArray<i32, D> {
    type Error = GeoArrowError;

    fn try_from(value: MultiPointArray<i64, D>) -> Result<Self> {
        Ok(Self::new(
            value.coords,
            offsets_buffer_i64_to_i32(&value.geom_offsets)?,
            value.validity,
            value.metadata,
        ))
    }
}

/// Default to an empty array
impl<O: OffsetSizeTrait, const D: usize> Default for MultiPointArray<O, D> {
    fn default() -> Self {
        MultiPointBuilder::default().into()
    }
}

impl<O: OffsetSizeTrait, const D: usize> PartialEq for MultiPointArray<O, D> {
    fn eq(&self, other: &Self) -> bool {
        if self.validity != other.validity {
            return false;
        }

        if !offset_buffer_eq(&self.geom_offsets, &other.geom_offsets) {
            return false;
        }

        if self.coords != other.coords {
            return false;
        }

        true
    }
}

impl<O: OffsetSizeTrait, const D: usize> TryFrom<MixedGeometryArray<O, D>>
    for MultiPointArray<O, D>
{
    type Error = GeoArrowError;

    fn try_from(value: MixedGeometryArray<O, D>) -> Result<Self> {
        if value.has_line_strings()
            || value.has_polygons()
            || value.has_multi_line_strings()
            || value.has_multi_polygons()
        {
            return Err(GeoArrowError::General("Unable to cast".to_string()));
        }

        if value.has_only_points() {
            return Ok(value.points.into());
        }

        if value.has_only_multi_points() {
            return Ok(value.multi_points);
        }

        let mut capacity = value.multi_points.buffer_lengths();
        // Hack: move to newtype
        capacity.coord_capacity += value.points.buffer_lengths();
        capacity.geom_capacity += value.points.buffer_lengths();

        let mut builder = MultiPointBuilder::<O, D>::with_capacity_and_options(
            capacity,
            value.coord_type(),
            value.metadata(),
        );
        value
            .iter()
            .try_for_each(|x| builder.push_geometry(x.as_ref()))?;
        Ok(builder.finish())
    }
}

impl<O: OffsetSizeTrait, const D: usize> TryFrom<GeometryCollectionArray<O, D>>
    for MultiPointArray<O, D>
{
    type Error = GeoArrowError;

    fn try_from(value: GeometryCollectionArray<O, D>) -> Result<Self> {
        MixedGeometryArray::try_from(value)?.try_into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::geoarrow_data::{
        example_multipoint_interleaved, example_multipoint_separated, example_multipoint_wkb,
    };
    use crate::test::multipoint::{mp0, mp1};

    #[test]
    fn geo_roundtrip_accurate() {
        let arr: MultiPointArray<i64, 2> = vec![mp0(), mp1()].as_slice().into();
        assert_eq!(arr.value_as_geo(0), mp0());
        assert_eq!(arr.value_as_geo(1), mp1());
    }

    #[test]
    fn geo_roundtrip_accurate_option_vec() {
        let arr: MultiPointArray<i64, 2> = vec![Some(mp0()), Some(mp1()), None].into();
        assert_eq!(arr.get_as_geo(0), Some(mp0()));
        assert_eq!(arr.get_as_geo(1), Some(mp1()));
        assert_eq!(arr.get_as_geo(2), None);
    }

    #[test]
    fn slice() {
        let arr: MultiPointArray<i64, 2> = vec![mp0(), mp1()].as_slice().into();
        let sliced = arr.slice(1, 1);
        assert_eq!(sliced.len(), 1);
        assert_eq!(sliced.get_as_geo(0), Some(mp1()));
    }

    #[test]
    fn owned_slice() {
        let arr: MultiPointArray<i64, 2> = vec![mp0(), mp1()].as_slice().into();
        let sliced = arr.owned_slice(1, 1);

        // assert!(
        //     !sliced.geom_offsets.buffer().is_sliced(),
        //     "underlying offsets should not be sliced"
        // );
        assert_eq!(arr.len(), 2);
        assert_eq!(sliced.len(), 1);
        assert_eq!(sliced.get_as_geo(0), Some(mp1()));
    }

    #[test]
    fn parse_wkb_geoarrow_interleaved_example() {
        let geom_arr = example_multipoint_interleaved();

        let wkb_arr = example_multipoint_wkb();
        let parsed_geom_arr: MultiPointArray<i64, 2> = wkb_arr.try_into().unwrap();

        assert_eq!(geom_arr, parsed_geom_arr);
    }

    #[test]
    fn parse_wkb_geoarrow_separated_example() {
        // TODO: support checking equality of interleaved vs separated coords
        let geom_arr = example_multipoint_separated().into_coord_type(CoordType::Interleaved);

        let wkb_arr = example_multipoint_wkb();
        let parsed_geom_arr: MultiPointArray<i64, 2> = wkb_arr.try_into().unwrap();

        assert_eq!(geom_arr, parsed_geom_arr);
    }
}
