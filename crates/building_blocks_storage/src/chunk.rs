mod serialization;

pub use serialization::*;

use crate::ArrayNx1;

use building_blocks_core::prelude::*;

/// One piece of a chunked lattice map.
pub trait Chunk {
    /// The inner array type. This makes it easier for `Chunk` implementations to satisfy access trait bounds by inheriting them
    /// from existing array types like `ArrayNx1`.
    type Array;

    /// Borrow the inner array.
    fn array_ref(&self) -> &Self::Array;

    /// Mutably borrow the inner array.
    fn array_mut(&mut self) -> &mut Self::Array;
}

impl<N, T> Chunk for ArrayNx1<N, T> {
    type Array = Self;

    #[inline]
    fn array_ref(&self) -> &Self::Array {
        self
    }

    #[inline]
    fn array_mut(&mut self) -> &mut Self::Array {
        self
    }
}

pub struct ChunkWithMeta<N, T, Meta> {
    pub array: ArrayNx1<N, T>,
    pub metadata: Meta,
}

impl<N, T, Meta> Chunk for ChunkWithMeta<N, T, Meta>
where
    PointN<N>: IntegerPoint<N>,
    T: Clone + Default,
    Meta: Default,
{
    type Array = ArrayNx1<N, T>;

    #[inline]
    fn array_ref(&self) -> &Self::Array {
        &self.array
    }

    #[inline]
    fn array_mut(&mut self) -> &mut Self::Array {
        &mut self.array
    }
}
