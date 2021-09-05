use crate::SmallVec;

// TODO not needed anymore
impl<T, const N: usize> Drop for SmallVec<T, N> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
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

    use crate::SmallVec;

    #[test]
    fn test_drop_locally() {
        let mut cnt = 0u8;
        let mut buf = SmallVec::<_, 3>::new();

        assert_eq!(cnt, 0);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 1);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 2);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 3);

        std::mem::drop(buf);
        assert_eq!(cnt, 0);
    }

    #[test]
    fn test_drop_remotely() {
        let mut cnt = 0u8;
        let mut buf = SmallVec::<_, 3>::new();

        assert_eq!(cnt, 0);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 1);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 2);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 3);

        buf.push(CounterGuard::new(&mut cnt));
        assert!(buf.is_remote());
        assert_eq!(cnt, 4);

        buf.push(CounterGuard::new(&mut cnt));
        assert_eq!(cnt, 5);

        std::mem::drop(buf);
        assert_eq!(cnt, 0);
    }
}