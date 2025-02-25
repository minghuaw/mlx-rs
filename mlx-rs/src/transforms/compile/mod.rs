//! Compilation of functions.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::{Closure, Guarded, VectorArray};
use crate::Array;

#[allow(clippy::module_inception)]
mod compile;
mod compile_with_state;

pub use compile::*;
pub use compile_with_state::*;

/// Globally enable the compilation of functions.
///
/// Default is enabled.
pub fn enable_compile() {
    unsafe {
        mlx_sys::mlx_enable_compile();
    }
}

/// Globally disable the compilation of functions.
///
/// Default is enabled.
pub fn disable_compile() {
    unsafe {
        mlx_sys::mlx_disable_compile();
    }
}

/// Clear the memory cache.
pub fn clear_cache() {
    unsafe {
        mlx_sys::mlx_detail_compile_clear_cache();
    }
}

/// A compiled function that can be called.
#[derive(Debug, Clone)]
pub struct Compiled<F, G> {
    f_marker: std::marker::PhantomData<F>,
    state: CompiledState<G>,
}

#[derive(Debug, Clone)]
struct CompiledState<F> {
    f: F,
    shapeless: bool,
    id: usize,
}

impl<F> Drop for CompiledState<F> {
    fn drop(&mut self) {
        unsafe {
            // remove the compiled structure from the back end
            mlx_sys::mlx_detail_compile_erase(self.id);
        }
    }
}

fn type_id_to_usize<T>(_val: &T) -> usize
where
    T: 'static,
{
    // hash type id to usize
    let type_id = std::any::TypeId::of::<T>();
    let mut hasher = DefaultHasher::new();
    type_id.hash(&mut hasher);
    hasher.finish() as usize
}

fn update_by_replace_with_ref_to_new_array(src: &mut Array, new_array: &Array) {
    unsafe {
        mlx_sys::mlx_array_set(&mut src.as_ptr() as *mut _, new_array.as_ptr());
    }
}
