from __future__ import annotations

from typing import Protocol, Self, Tuple

import numpy as np
from numpy.typing import NDArray

class _ArrowArrayExportable(Protocol):
    """A GeoArrow array from a local or remote (e.g. geoarrow.c) source."""
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...

class PointArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def area(self) -> Float64Array: ...
    def center(self) -> PointArray: ...
    def centroid(self) -> PointArray: ...
    def convex_hull(self) -> PolygonArray: ...
    @classmethod
    def from_wkb(cls, array: _ArrowArrayExportable) -> Self: ...
    def to_wkb(self) -> WKBArray: ...

class LineStringArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def area(self) -> Float64Array: ...
    def center(self) -> PointArray: ...
    def centroid(self) -> PointArray: ...
    # def chaikin_smoothing(self, n_iterations: int) -> Self: ...
    def convex_hull(self) -> PolygonArray: ...
    @classmethod
    def from_wkb(cls, array: _ArrowArrayExportable) -> Self: ...
    def to_wkb(self) -> WKBArray: ...

class PolygonArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def area(self) -> Float64Array: ...
    def center(self) -> PointArray: ...
    def centroid(self) -> PointArray: ...
    # def chaikin_smoothing(self, n_iterations: int) -> Self: ...
    def convex_hull(self) -> PolygonArray: ...
    @classmethod
    def from_wkb(cls, array: _ArrowArrayExportable) -> Self: ...
    def to_wkb(self) -> WKBArray: ...

class MultiPointArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def area(self) -> Float64Array: ...
    def center(self) -> PointArray: ...
    def centroid(self) -> PointArray: ...
    def convex_hull(self) -> PolygonArray: ...
    @classmethod
    def from_wkb(cls, array: _ArrowArrayExportable) -> Self: ...
    def to_wkb(self) -> WKBArray: ...

class MultiLineStringArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def area(self) -> Float64Array: ...
    def center(self) -> PointArray: ...
    def centroid(self) -> PointArray: ...
    # def chaikin_smoothing(self, n_iterations: int) -> Self: ...
    def convex_hull(self) -> PolygonArray: ...
    @classmethod
    def from_wkb(cls, array: _ArrowArrayExportable) -> Self: ...
    def to_wkb(self) -> WKBArray: ...

class MultiPolygonArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def area(self) -> Float64Array: ...
    def center(self) -> PointArray: ...
    def centroid(self) -> PointArray: ...
    # def chaikin_smoothing(self, n_iterations: int) -> Self: ...
    def convex_hull(self) -> PolygonArray: ...
    @classmethod
    def from_wkb(cls, array: _ArrowArrayExportable) -> Self: ...
    def to_wkb(self) -> WKBArray: ...

class MixedGeometryArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def area(self) -> Float64Array: ...
    def center(self) -> PointArray: ...
    def centroid(self) -> PointArray: ...
    def convex_hull(self) -> PolygonArray: ...
    # @classmethod
    # def from_wkb(cls, array: _ArrowArrayExportable) -> Self: ...
    def to_wkb(self) -> WKBArray: ...

class GeometryCollectionArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def area(self) -> Float64Array: ...
    def center(self) -> PointArray: ...
    def centroid(self) -> PointArray: ...
    def convex_hull(self) -> PolygonArray: ...
    # @classmethod
    # def from_wkb(cls, array: _ArrowArrayExportable) -> Self: ...
    def to_wkb(self) -> WKBArray: ...

class WKBArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...

class BooleanArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...

class Float16Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...

class Float32Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.float32]: ...

class Float64Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.float64]: ...

class Int16Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.int16]: ...

class Int32Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.int32]: ...

class Int64Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.int64]: ...

class Int8Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.int8]: ...

class LargeStringArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...

class StringArray:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...

class UInt16Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.uint16]: ...

class UInt32Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.uint32]: ...

class UInt64Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.uint64]: ...

class UInt8Array:
    def __len__(self) -> int: ...
    def __arrow_c_array__(
        self, requested_schema: object | None = None
    ) -> Tuple[object, object]: ...
    def to_numpy(self) -> NDArray[np.uint8]: ...

class ChunkedPointArray:
    def __len__(self) -> int: ...
    def area(self) -> ChunkedFloat64Array: ...
    def center(self) -> ChunkedPointArray: ...
    def centroid(self) -> ChunkedPointArray: ...
    def convex_hull(self) -> ChunkedPolygonArray: ...

class ChunkedLineStringArray:
    def __len__(self) -> int: ...
    def area(self) -> ChunkedFloat64Array: ...
    def center(self) -> ChunkedPointArray: ...
    def centroid(self) -> ChunkedPointArray: ...
    # def chaikin_smoothing(self, n_iterations: int) -> Self: ...
    def convex_hull(self) -> ChunkedPolygonArray: ...

class ChunkedPolygonArray:
    def __len__(self) -> int: ...
    def area(self) -> ChunkedFloat64Array: ...
    def center(self) -> ChunkedPointArray: ...
    def centroid(self) -> ChunkedPointArray: ...
    # def chaikin_smoothing(self, n_iterations: int) -> Self: ...
    def convex_hull(self) -> ChunkedPolygonArray: ...

class ChunkedMultiPointArray:
    def __len__(self) -> int: ...
    def area(self) -> ChunkedFloat64Array: ...
    def center(self) -> ChunkedPointArray: ...
    def centroid(self) -> ChunkedPointArray: ...
    def convex_hull(self) -> ChunkedPolygonArray: ...

class ChunkedMultiLineStringArray:
    def __len__(self) -> int: ...
    def area(self) -> ChunkedFloat64Array: ...
    def center(self) -> ChunkedPointArray: ...
    def centroid(self) -> ChunkedPointArray: ...
    # def chaikin_smoothing(self, n_iterations: int) -> Self: ...
    def convex_hull(self) -> ChunkedPolygonArray: ...

class ChunkedMultiPolygonArray:
    def __len__(self) -> int: ...
    def area(self) -> ChunkedFloat64Array: ...
    def center(self) -> ChunkedPointArray: ...
    def centroid(self) -> ChunkedPointArray: ...
    # def chaikin_smoothing(self, n_iterations: int) -> Self: ...
    def convex_hull(self) -> ChunkedPolygonArray: ...

class ChunkedMixedGeometryArray:
    def __len__(self) -> int: ...
    def area(self) -> ChunkedFloat64Array: ...
    def center(self) -> ChunkedPointArray: ...
    def centroid(self) -> ChunkedPointArray: ...
    def convex_hull(self) -> ChunkedPolygonArray: ...

class ChunkedGeometryCollectionArray:
    def __len__(self) -> int: ...
    def area(self) -> ChunkedFloat64Array: ...
    def center(self) -> ChunkedPointArray: ...
    def centroid(self) -> ChunkedPointArray: ...
    def convex_hull(self) -> ChunkedPolygonArray: ...

class ChunkedWKBArray:
    def __len__(self) -> int: ...

class ChunkedBooleanArray:
    def __len__(self) -> int: ...

class ChunkedFloat16Array:
    def __len__(self) -> int: ...

class ChunkedFloat32Array:
    def __len__(self) -> int: ...

class ChunkedFloat64Array:
    def __len__(self) -> int: ...

class ChunkedInt16Array:
    def __len__(self) -> int: ...

class ChunkedInt32Array:
    def __len__(self) -> int: ...

class ChunkedInt64Array:
    def __len__(self) -> int: ...

class ChunkedInt8Array:
    def __len__(self) -> int: ...

class ChunkedLargeStringArray:
    def __len__(self) -> int: ...

class ChunkedStringArray:
    def __len__(self) -> int: ...

class ChunkedUInt16Array:
    def __len__(self) -> int: ...

class ChunkedUInt32Array:
    def __len__(self) -> int: ...

class ChunkedUInt64Array:
    def __len__(self) -> int: ...

class ChunkedUInt8Array:
    def __len__(self) -> int: ...

def area(input: _ArrowArrayExportable) -> Float64Array: ...
def signed_area(input: _ArrowArrayExportable) -> Float64Array: ...
def center(input: _ArrowArrayExportable) -> PointArray: ...
def centroid(input: _ArrowArrayExportable) -> PointArray: ...
def convex_hull(input: _ArrowArrayExportable) -> PolygonArray: ...
def from_wkb(
    input: _ArrowArrayExportable,
) -> (
    PointArray
    | LineStringArray
    | PolygonArray
    | MultiPointArray
    | MultiLineStringArray
    | MultiPolygonArray
    | MixedGeometryArray
    | GeometryCollectionArray
): ...
def to_wkb(input: _ArrowArrayExportable) -> WKBArray: ...
