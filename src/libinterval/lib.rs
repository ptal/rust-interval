// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library proposes structures for [interval arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic), most [set operations](https://en.wikipedia.org/wiki/Set_%28mathematics%29) are implemented. A second part of this library defines a bunch of traits for programming generic operations on collections. This part might be moved into another library if it proves its usefulness. Or it might be removed when proper generic support will land in the standard collection.
//!
//! # Overflow behavior
//!
//! Nothing special is done against overflow beside the checks done by Rust in debug mode for arithmetic overflows.
//!
//! # Limits
//!
//! Interval bounds must implement, for most operations, the `Width` trait. This is because the maximum size of an n-bits interval can not fit in an n-bits integer. Consider the interval `[0..1]` with 1-bit bounds, the size `2` can not be represented with only one bit. It needs `n+1` bits, and this is problematic with the largest primitive types such as `u64`. Therefore, the interval bounds must be used within the limits of `Width::min_value()` and `Width::max_value()`, and not by the limits provided by `num::traits::Bounded`.
//!
//! # Examples
//!
//! For examples see the [interval module](interval/index.html), [interval set module](interval_set/index.html) or the [ncollections module](ncollections/index.html).
//!
//! # References
//! * [Boost Interval Arithmetic Library](http://www.boost.org/doc/libs/1_57_0/libs/numeric/interval/doc/interval.html)
//! * [Boost Interval Container Library](http://www.boost.org/doc/libs/1_57_0/libs/icl/doc/html/index.html)
//! * T.J. Hickey, Qun Ju, and M.H. van Emden. Interval arithmetic: from principles to implementation. Journal of the ACM, 48(5):1038-1068, 2001.
//!

#![crate_name = "interval"]
#![crate_type = "dylib"]

#![feature(collections, hashmap_hasher, enumset, peekable_is_empty)]

extern crate collections;
extern crate num;
extern crate bit_set;

#[macro_use]
pub mod ncollections;
pub mod interval;
pub mod interval_set;
pub mod ops;

pub use interval::Interval;
pub use interval_set::IntervalSet;
