pub struct Chunk<I: Iterator, const N: usize> {
    iter: I,
}

pub trait ChunkOps: Iterator + Sized {
    fn chunk<const N: usize>(self) -> Chunk<Self, N>;
}

impl<I: Iterator> ChunkOps for I {
    fn chunk<const N: usize>(self) -> Chunk<Self, N> {
        Chunk::<Self, N> { iter: self }
    }
}

impl<I: Iterator> Iterator for Chunk<I, 2> {
    type Item = [I::Item; 2];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        Some([a, b])
    }
}

impl<I: Iterator> Iterator for Chunk<I, 3> {
    type Item = [I::Item; 3];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let b = self.iter.next()?;
        let c = self.iter.next()?;
        Some([a, b, c])
    }
}
