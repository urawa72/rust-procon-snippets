use cargo_snippet::snippet;

#[allow(unused_macros)]
#[snippet("min")]
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

#[allow(unused_macros)]
#[snippet("chmin")]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
#[snippet("max")]
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

#[allow(unused_macros)]
#[snippet("chmax")]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

#[test]
fn test_chmin() {
    let mut a = 10;
    let b = 13;
    let c = 5;
    let result = chmin!(a, b, c);
    assert_eq!(result, true);
    assert_eq!(a, 5);
}

#[test]
fn test_chmax() {
    let mut a = 10;
    let b = 13;
    let c = 5;
    let result = chmax!(a, b, c);
    assert_eq!(result, true);
    assert_eq!(a, 13);
}
