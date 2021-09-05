use crate::SmallVec;
use std::ops::{Index, IndexMut};

impl<T, const N: usize> Index<usize> for SmallVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::LocalBuf(local_vec) => &local_vec[index],
            Self::RemoteBuf(vec) => &vec[index],
        }
    }
}