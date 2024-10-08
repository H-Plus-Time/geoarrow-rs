use crate::array::SeparatedCoordBuffer;
use geos::CoordSeq;

impl<const D: usize> TryFrom<SeparatedCoordBuffer<D>> for CoordSeq {
    type Error = geos::Error;

    fn try_from(value: SeparatedCoordBuffer<D>) -> std::result::Result<Self, geos::Error> {
        match D {
            2 => CoordSeq::new_from_arrays(&value.buffers[0], &value.buffers[1], None, None),
            3 => CoordSeq::new_from_arrays(
                &value.buffers[0],
                &value.buffers[1],
                Some(&value.buffers[2]),
                None,
            ),
            _ => panic!(),
        }
    }
}
