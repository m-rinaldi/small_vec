mod drop;

use local_vec::LocalVec;


/// A vector supporting small buffer optimization
pub enum SmallVec<T, const N: usize> {
    LocalBuf(LocalVec<T, N>),
    RemoteBuf(Vec<T>),
}

use SmallVec::*;

impl<T, const N: usize> SmallVec<T, N> {
    pub fn new() -> Self {
        LocalBuf(LocalVec::new())
    }

    pub fn len(&self) -> usize {
        match self {
            LocalBuf(vec) => vec.len(),
            RemoteBuf(vec) => vec.len(),
        }
    }

    /// whether the vector's elements are in the local buffer
    pub fn is_local(&self) -> bool {
        match self {
            LocalBuf(_) => true,
            RemoteBuf(_) => false,
        }
    }

    /// whether the vector's elements are in the remote buffer
    pub fn is_remote(&self) -> bool {
        return !self.is_local()
    }
    
    pub fn push(&mut self, val: T) {
        match self {
            LocalBuf(local_vec) => {
                if !local_vec.is_full() {
                    // there is still room in the local buffer
                    local_vec.push(val);
                } else {
                    // need to allocate a remote buffer
                    let cap = {
                        // new capacity to be set to the previous one plus
                        // one for the new element to be pushed
                        local_vec.len()
                            .checked_add(1)
                            .expect("new capacity would overflow capacity type")
                    };
                    let mut vec = Vec::with_capacity(cap);

                    // TODO iterate instead directly on the LocalVec
                    // move the elements from the local to the remote buffer
                    let arr: [T; N] = unsafe {
                        std::mem::transmute_copy(local_vec)
                    };

                    for elem in arr {
                        vec.push(elem);
                    }

                    // TODO use local_vec.set_len(0) instead
                    // prevent local_vec's elements to be dropped twice
                    while let Some(val) = local_vec.pop() {
                        std::mem::forget(val);
                    }

                    // replace old buffer by new one
                    *self = RemoteBuf(vec);

                    // push the new element
                    self.push(val);
                }
            },
            RemoteBuf(vec) => {
                vec.push(val);
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            LocalBuf(vec) => vec.pop(),
            RemoteBuf(vec) => vec.pop(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SmallVec;

    #[test]
    fn test_new() {
        SmallVec::<u8, 32>::new();
    }

    #[test]
    fn test_zero_len() {
        let buf = SmallVec::<u8, 32>::new();
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_switch_from_local_to_remote() {
        let mut buf = SmallVec::<usize, 4>::new();
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
        let mut buf = SmallVec::<_, 4>::new();

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
        let mut buf = SmallVec::<_, 4>::new();

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
