#![deny(unused_unsafe, missing_debug_implementations)]

mod array;
mod device;
mod dtype;
pub mod error;
pub mod fft;
pub mod ops;
mod stream;
mod utils;

pub use array::*;
pub use device::*;
pub use dtype::*;
pub use stream::*;

// TODO: what to put in the prelude?
pub mod prelude {
    pub use crate::{
        array::Array, dtype::Dtype, ops::indexing::{IndexOp, NewAxis, Ellipsis, IntoStrideBy},
    };
}

pub(crate) mod sealed {
    /// A marker trait to prevent external implementations of the `Sealed` trait.
    pub trait Sealed {}
}
