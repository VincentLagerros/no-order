/// A helper struct for no ordering, where every instance of NoOrder<T> is equivalent to any other NoOrder<T>.
///
/// This struct is a helper to be used with data structures like [`std::collections::BinaryHeap`] or [`std::collections::HashSet`],
/// and can be used to ignore ordering or hashing of part of a key.
///
/// Can be used for faster performance or when an key does not implement [`std::cmp::Ord`] or [`std::hash::Hash`],
/// and is not relavant.
///
/// # Examples
///
/// ```
/// use no_order::NoOrder;
/// use std::collections::BinaryHeap;
/// use std::collections::HashSet;
///
/// // Any NoOrder is equivalent to any other.
/// assert_eq!(NoOrder(1), NoOrder(2));
/// assert_eq!(NoOrder(1), NoOrder(1));
///
/// // This means that they also share the same key.
/// let set = HashSet::from([NoOrder(1), NoOrder(1), NoOrder(2), NoOrder(3)]);
/// assert_eq!(set.len(), 1);
///
/// // And can be used to ignore the the second value in a tuple.
/// let set_tuple = HashSet::from([(1, NoOrder(1)), (2, NoOrder(1)), (2, NoOrder(2))]);
/// assert_eq!(set_tuple.len(), 2);
///
/// // When used in a heap or tree like data structure, we can store extra data without sorting on it.
/// // This means that we can use types that do not traditionally implement Ord.
/// let mut heap = BinaryHeap::new();
/// heap.push((2, NoOrder(1.24)));
/// heap.push((1, NoOrder(2.15)));
/// heap.push((2, NoOrder(3.74)));
/// heap.push((2, NoOrder(2.96)));
///
/// // It only acts as a max heap on the first value,
/// // and the ordering on the second value is undefiend.
/// while let Some((a, NoOrder(b))) = heap.pop() {
///     println!("{a}, {b}");
/// }
/// ```
#[repr(transparent)]
#[derive(Debug, Copy, Default)]
pub struct NoOrder<T>(pub T);

impl<T> PartialEq for NoOrder<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> PartialOrd for NoOrder<T> {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl<T> Eq for NoOrder<T> {}

impl<T> Ord for NoOrder<T> {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl<T> std::hash::Hash for NoOrder<T> {
    fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
}

impl<T: Clone> Clone for NoOrder<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0)
    }
}
