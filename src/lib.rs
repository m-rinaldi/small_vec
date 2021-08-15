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
        // TODO use to-date unstable feature uninit_array() instead
        Self::Local(uninit_arr, 0)
    }

    pub fn len(&self) -> usize {
        use SmallBuf::*;
        match self {
            Local(_,len) => *len,
            Remote(vec) => vec.len(),
        }
    }

    // TODO
    //pub fn is_local(&self) -> bool
    //pub fn is_remote(&self) -> bool

    pub fn push(&mut self, val: T) {
        use SmallBuf::*;
        match self {
            Local(arr, len) => {
                if *len < N {
                    arr[*len] = MaybeUninit::new(val);
                    // TODO what if panic here?
                    //  drop all stored elements
                    *len += 1;
                } else {
                    let vec = {
                        let buf: [T; N] = unsafe {
                            std::mem::transmute_copy(arr)
                        };
                        Vec::from(buf)
                    };
                    *self = Remote(vec);
                    self.push(val);
                }
            },
            Remote(vec) => {
                vec.push(val);
            },
        }
    }

    /* TODO
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

    #[test]
    fn test_switch_from_local_to_remote() {
        let mut buf = SmallBuf::<usize, 4>::new();
        assert_eq!(buf.len(), 0);

        buf.push(1);
        assert_eq!(buf.len(), 1);

        buf.push(2);
        assert_eq!(buf.len(), 2);

        buf.push(3);
        assert_eq!(buf.len(), 3);

        buf.push(4);
        assert_eq!(buf.len(), 4);

        buf.push(5);
        assert_eq!(buf.len(), 5);
    }
}
