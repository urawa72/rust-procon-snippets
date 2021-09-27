use cargo_snippet::snippet;

#[snippet("BinarySearch")]
use std::cmp::Ordering;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

#[snippet("BinarySearch")]
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => low = mid + 1,
                Ordering::Equal | Ordering::Greater => high = mid,
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        low
    }
}

#[test]
fn test_binary_search() {
    let v = vec![2, 4, 5, 8, 9, 44, 500];

    assert_eq!(v.lower_bound(&5), 2);
    assert_eq!(v.upper_bound(&5), 3);
    assert_eq!(v.lower_bound(&500), 6);
    assert_eq!(v.lower_bound(&500), 6);
    assert_eq!(v.lower_bound(&0), 0);
    assert_eq!(v.lower_bound(&0), 0);
    assert_eq!(v.lower_bound(&666), 7);
    assert_eq!(v.upper_bound(&666), 7);
}
