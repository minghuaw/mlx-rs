use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

use mlx_sys::mlx_array;

use crate::error::{OperationError, ReshapeError};
use crate::Array;

/// Helper method to get a string representation of an mlx object.
pub(crate) fn mlx_describe(ptr: *mut ::std::os::raw::c_void) -> Option<String> {
    let mlx_description = unsafe { mlx_sys::mlx_tostring(ptr) };
    let c_str = unsafe { mlx_sys::mlx_string_data(mlx_description) };

    let description = if c_str.is_null() {
        None
    } else {
        Some(unsafe {
            std::ffi::CStr::from_ptr(c_str)
                .to_string_lossy()
                .into_owned()
        })
    };

    unsafe { mlx_sys::mlx_free(mlx_description as *mut std::ffi::c_void) };

    description
}

pub(crate) fn resolve_index_unchecked(index: i32, len: usize) -> usize {
    if index.is_negative() {
        (len as i32 + index) as usize
    } else {
        index as usize
    }
}

pub(crate) fn resolve_index(index: i32, len: usize) -> Option<usize> {
    let abs_index = index.abs() as usize;

    if index.is_negative() {
        if abs_index <= len {
            Some(len - abs_index)
        } else {
            None
        }
    } else {
        if abs_index < len {
            Some(abs_index)
        } else {
            None
        }
    }
}

pub(crate) fn all_unique(arr: &[i32]) -> Result<(), i32> {
    let mut unique = std::collections::HashSet::new();
    for &x in arr {
        if !unique.insert(x) {
            return Err(x);
        }
    }

    Ok(())
}

/// Helper method to check if two arrays are broadcastable.
///
/// Uses the same broadcasting rules as numpy.
/// https://numpy.org/doc/1.20/user/theory.broadcasting.html
///
/// "The size of the trailing axes for both arrays in an operation must
/// either be the same size or one of them must be one."
pub(crate) fn is_broadcastable(a: &[i32], b: &[i32]) -> bool {
    a.iter()
        .rev()
        .zip(b.iter().rev())
        .all(|(a, b)| *a == 1 || *b == 1 || a == b)
}

pub(crate) fn can_reduce_shape(shape: &[i32], axes: &[i32]) -> Result<(), OperationError> {
    let ndim = shape.len() as i32;
    let mut axes_set = std::collections::HashSet::new();
    for &axis in axes {
        let ax = if axis < 0 { axis + ndim } else { axis };
        if ax < 0 || ax >= ndim {
            return Err(OperationError::AxisOutOfBounds {
                axis,
                dim: shape.len(),
            });
        }

        axes_set.insert(ax);
    }

    if axes_set.len() != axes.len() {
        return Err(OperationError::WrongInput(format!(
            "Duplicate axes in {:?}",
            axes
        )));
    }

    Ok(())
}

impl Array {
    /// Helper method to check if an array can be reshaped to a given shape.
    pub fn can_reshape_to<'a>(&self, shape: &'a [i32]) -> Result<(), ReshapeError<'a>> {
        if self.shape() == shape {
            return Ok(());
        }

        let mut size = 1;
        let mut infer_idx: isize = -1;
        for i in 0..shape.len() {
            if shape[i] == -1 {
                if infer_idx >= 0 {
                    return Err(ReshapeError::MultipleInferredDims);
                }

                infer_idx = i as isize;
            } else {
                size *= shape[i];
            }
        }

        if size > 0 {
            let quotient = self.size() / size as usize;
            if infer_idx >= 0 {
                size *= quotient as i32;
            }
        } else if infer_idx >= 0 {
            return Err(ReshapeError::EmptyArray);
        }

        // validate the reshaping is valid
        if self.size() != size as usize {
            return Err(ReshapeError::InvalidShape {
                size: self.size(),
                shape,
            });
        }

        Ok(())
    }

    /// Helper method to validate an axis is in bounds.
    pub fn validate_axis_in_bounds(&self, axis: Option<i32>) -> Result<(), OperationError> {
        if let Some(axis) = axis {
            if axis >= self.ndim() as i32 || axis < -(self.ndim() as i32) {
                return Err(OperationError::AxisOutOfBounds {
                    axis,
                    dim: self.ndim(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_broadcastable() {
        let a = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        let b = Array::from_slice(&[2.0, 2.0, 2.0], &[3]);
        assert!(is_broadcastable(a.shape(), b.shape()));

        let a = Array::from_slice(
            &[
                0.0, 0.0, 0.0, 10.0, 10.0, 10.0, 20.0, 20.0, 20.0, 30.0, 30.0, 30.0,
            ],
            &[4, 3],
        );
        let b = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        assert!(is_broadcastable(a.shape(), b.shape()));

        let a = Array::from_slice(
            &[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
            ],
            &[2, 2, 4],
        );
        let b = Array::from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], &[2, 4]);
        assert!(is_broadcastable(a.shape(), b.shape()));
    }

    #[test]
    fn test_is_broadcastable_scalar() {
        let a = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        let b: Array = 2.0.into();
        assert!(is_broadcastable(a.shape(), b.shape()));
    }

    #[test]
    fn test_is_broadcastable_empty() {
        let a = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        assert!(is_broadcastable(&[], a.shape()));
    }

    #[test]
    fn test_not_broadcastable() {
        let a = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        let b = Array::from_slice(&[2.0, 2.0, 2.0, 2.0], &[4]);
        assert!(!is_broadcastable(a.shape(), b.shape()));

        let a = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        let b = Array::from_slice(&[2.0, 2.0], &[1, 2]);
        assert!(!is_broadcastable(a.shape(), b.shape()));
    }

    #[test]
    fn test_can_reshape_to() {
        let a = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        assert!(a.can_reshape_to(&[3]).is_ok());
        assert!(a.can_reshape_to(&[1, 3]).is_ok());
        assert!(a.can_reshape_to(&[3, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 3]).is_ok());
        assert!(a.can_reshape_to(&[1, 3, 1]).is_ok());
        assert!(a.can_reshape_to(&[3, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 1, 3]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 3, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 3, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[3, 1, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 1, 1, 3]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 1, 3, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 3, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 3, 1, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[3, 1, 1, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 1, 1, 1, 3]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 1, 1, 3, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 1, 3, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 1, 3, 1, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[1, 3, 1, 1, 1, 1]).is_ok());
        assert!(a.can_reshape_to(&[3, 1, 1, 1, 1, 1]).is_ok());
    }

    #[test]
    fn test_reshape_negative_dim() {
        let a = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        assert!(a.can_reshape_to(&[1, -1]).is_ok());
        assert!(a.can_reshape_to(&[-1, 1]).is_ok());
        assert!(a.can_reshape_to(&[-1]).is_ok());
        assert!(a.can_reshape_to(&[1, -1, 1]).is_ok());
        assert!(a.can_reshape_to(&[-1, 1, 1]).is_ok());

        assert!(!a.can_reshape_to(&[1, -2]).is_ok());
    }

    #[test]
    fn test_cannot_reshape_to() {
        let a = Array::from_slice(&[1.0, 2.0, 3.0], &[3]);
        assert!(!a.can_reshape_to(&[2]).is_ok());
        assert!(!a.can_reshape_to(&[2, 2]).is_ok());
        assert!(!a.can_reshape_to(&[2, 2, 2]).is_ok());
        assert!(!a.can_reshape_to(&[2, 2, 2, 2]).is_ok());
        assert!(!a.can_reshape_to(&[2, 2, 2, 2, 2]).is_ok());
        assert!(!a.can_reshape_to(&[2, 2, 2, 2, 2, 2]).is_ok());
    }
}
