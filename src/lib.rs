use std::mem::MaybeUninit;

pub enum SmallBuf<T, const N: usize> {
    Local([MaybeUninit<T>; N], usize),
    Remote(Vec<T>),
}

impl<T, const N: usize> SmallBuf<T, N> {
    pub fn new() -> Self {
        let uninit_arr:[MaybeUninit<T>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        // TODO use uninit_array() instead
        Self::Local(uninit_arr, 0)
    }

    pub fn len(&self) -> usize {
        use SmallBuf::*;
        match self {
            Local(_,len) => *len,
            Remote(vec) => vec.len(),
        }
    }

    /* TODO
    pub fn push(&mut self, val: T) {

    }

    pub fn pop(&mut self) -> Option<T> {

    }
    */
}

#[cfg(test)]
mod tests {
    use crate::SmallBuf;

    #[test]
    fn test_new() {
        SmallBuf::<u8, 32>::new();
    }

    #[test]
    fn test_zero_len() {
        let buf = SmallBuf::<u8, 32>::new();
        assert_eq!(buf.len(), 0);
    }
}
