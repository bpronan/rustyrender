/// Implementation of max since std::cmp::max doesn't work for f32
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

/// Implementation of max since std::cmp::max doesn't work for f32
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

pub(crate) use max;
pub(crate) use min;
