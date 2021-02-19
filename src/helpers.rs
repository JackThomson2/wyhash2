#[cfg(feature = "nightly")]
use std::intrinsics::{likely, unlikely};

#[cfg(feature = "nightly")]
macro_rules! likely {
    ($x:expr) => {
        likely($x)
    };
}

#[cfg(feature = "nightly")]
macro_rules! unlikely {
    ($x:expr) => {
        unlikely($x)
    };
}

#[cfg(not(feature = "nightly"))]
macro_rules! likely {
    ($x:expr) => {
        $x
    };
}

#[cfg(not(feature = "nightly"))]
macro_rules! unlikely {
    ($x:expr) => {
        $x
    };
}
