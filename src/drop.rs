use crate::SmallBuf;

impl<T, const N: usize> Drop for SmallBuf<T, N> {
    fn drop(&mut self) {
        match self {
            Self::Local(_, _) => {
                while let Some(val) = self.pop() {
                    // superfluous
                    std::mem::drop(val);
                }
            }
            Self::Remote(_) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    struct CounterGuard(*mut u8);

    impl<'a> CounterGuard {
        pub fn new(cnt: &'a mut u8) -> CounterGuard {
            *cnt += 1;
            CounterGuard(cnt as *mut u8)
        }
    }

    impl<'a> Drop for CounterGuard {
        fn drop(&mut self) {
            unsafe {
                *self.0 -= 1;
            }
        }
    }

    #[test]
    fn test_drop_locally() {
        let mut cnt = 0u8;
        let mut buf = SmallBuf<_, 3>::new();
    }
}