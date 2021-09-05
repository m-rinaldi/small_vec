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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let mut vec = SmallVec::<_, 4>::new();
        vec.push(0);
        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert!(vec.is_local());
        assert_eq!(vec[0], 0);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[2], 2);
        assert_eq!(vec[3], 3);

        vec.push(4);
        vec.push(5);
        vec.push(6);
        assert!(vec.is_remote());

        assert_eq!(vec[0], 0);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[2], 2);
        assert_eq!(vec[3], 3);
        assert_eq!(vec[4], 4);
        assert_eq!(vec[5], 5);
        assert_eq!(vec[6], 6);

    }

    // TODO test out of bounds
}