mod drop;

use std::mem::MaybeUninit;

pub enum SmallBuf<T, const N: usize> {
    Local([MaybeUninit<T>; N], usize),
    Remote(Vec<T>),
}

use SmallBuf::*;

impl<T, const N: usize> SmallBuf<T, N> {
    pub fn new() -> Self {
        let uninit_arr:[MaybeUninit<T>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        // TODO use to-date unstable feature uninit_array() instead
        Local(uninit_arr, 0)
    }

    pub fn len(&self) -> usize {
        match self {
            Local(_,len) => *len,
            Remote(vec) => vec.len(),
        }
    }

    pub fn is_local(&self) -> bool {
        match self {
            Local(_, _) => true,
            Remote(_) => false,
        }
    }

    pub fn is_remote(&self) -> bool {
        return !self.is_local()
    }
    
    pub fn push(&mut self, val: T) {
        match self {
            Local(arr, len) => {
                if *len < N {
                    arr[*len] = MaybeUninit::new(val);
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

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Local(arr, len) => {
                if *len == 0 {
                    return None
                } else {
                    let val:T = unsafe {
                        std::mem::transmute_copy(&arr[*len-1])
                    };
                    *len -= 1;
                    Some(val)
                }
            }
            Remote(vec) => vec.pop(),
        }
    }
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
        assert!(buf.is_local());

        buf.push(2);
        assert_eq!(buf.len(), 2);
        assert!(buf.is_local());

        buf.push(3);
        assert_eq!(buf.len(), 3);
        assert!(buf.is_local());

        buf.push(4);
        assert_eq!(buf.len(), 4);
        assert!(buf.is_local());

        buf.push(5);
        assert_eq!(buf.len(), 5);
        assert!(buf.is_remote());

        buf.push(6);
        assert_eq!(buf.len(), 6);
        assert!(buf.is_remote());
    }

    #[test]
    fn test_push_and_pop_locally() {
        let mut buf = SmallBuf::<_, 4>::new();

        buf.push(1usize);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        assert!(buf.is_local());

        assert_eq!(buf.pop(), Some(4));
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(1));
        assert_eq!(buf.pop(), None);
        assert_eq!(buf.pop(), None);
    }

    #[test]
    fn test_push_and_pop_remotely() {
        let mut buf = SmallBuf::<_, 4>::new();

        buf.push(1usize);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        assert_eq!(buf.len(), 4);
        assert!(buf.is_local());

        buf.push(5);
        assert_eq!(buf.len(), 5);
        assert!(buf.is_remote());

        buf.push(6);
        buf.push(7);
        assert_eq!(buf.len(), 7);

        assert_eq!(buf.pop(), Some(7));
        assert_eq!(buf.pop(), Some(6));
        assert_eq!(buf.pop(), Some(5));
        assert_eq!(buf.pop(), Some(4));
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(1));
        assert_eq!(buf.pop(), None);
        assert_eq!(buf.pop(), None);
    }
}
