use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let l = &self.0;
        let r = &other.0;
        if l == r {
            Ordering::Equal
        } else if l < r {
            Ordering::Greater
        } else if l > r {
            Ordering::Less
        } else if l.ne(l) && r.ne(r) {
            // Sort NaN as equal to NaN.
            Ordering::Equal
        } else if l.ne(l) {
            // Sort NaN less than non-NaN, so that it is last in the order.
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
