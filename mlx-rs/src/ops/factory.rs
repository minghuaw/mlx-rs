use crate::array::ArrayElement;
use crate::error::Result;
use crate::utils::guard::Guarded;
use crate::{array::Array, stream::StreamOrDevice};
use crate::{Dtype, Stream};
use mlx_internal_macros::default_device;
use num_traits::NumCast;

impl Array {
    /// Construct an array of zeros returning an error if shape is invalid.
    ///
    /// # Params
    ///
    /// - shape: Desired shape
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    /// Array::zeros_device::<f32>(&[5, 10], StreamOrDevice::default()).unwrap();
    /// ```
    #[default_device]
    pub fn zeros_device<T: ArrayElement>(
        shape: &[i32],
        stream: impl AsRef<Stream>,
    ) -> Result<Array> {
        let dtype = T::DTYPE;
        zeros_dtype_device(shape, dtype, stream)
    }

    /// Construct an array of ones returning an error if shape is invalid.
    ///
    /// # Params
    ///
    /// - shape: Desired shape
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    /// Array::ones_device::<f32>(&[5, 10], StreamOrDevice::default()).unwrap();
    /// ```
    #[default_device]
    pub fn ones_device<T: ArrayElement>(
        shape: &[i32],
        stream: impl AsRef<Stream>,
    ) -> Result<Array> {
        let dtype = T::DTYPE;
        ones_dtype_device(shape, dtype, stream)
    }

    /// Create an identity matrix or a general diagonal matrix returning an error if params are invalid.
    ///
    /// # Params
    ///
    /// - n: number of rows in the output
    /// - m: number of columns in the output -- equal to `n` if not specified
    /// - k: index of the diagonal - defaults to 0 if not specified
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    /// //  create [10, 10] array with 1's on the diagonal.
    /// let r = Array::eye_device::<f32>(10, None, None, StreamOrDevice::default()).unwrap();
    /// ```
    #[default_device]
    pub fn eye_device<T: ArrayElement>(
        n: i32,
        m: Option<i32>,
        k: Option<i32>,
        stream: impl AsRef<Stream>,
    ) -> Result<Array> {
        Array::try_from_op(|res| unsafe {
            mlx_sys::mlx_eye(
                res,
                n,
                m.unwrap_or(n),
                k.unwrap_or(0),
                T::DTYPE.into(),
                stream.as_ref().as_ptr(),
            )
        })
    }

    /// Construct an array with the given value returning an error if shape is invalid.
    ///
    /// Constructs an array of size `shape` filled with `values`. If `values`
    /// is an [Array] it must be [broadcasting](https://swiftpackageindex.com/ml-explore/mlx-swift/main/documentation/mlx/broadcasting) to the given `shape`.
    ///
    /// # Params
    ///
    /// - shape: shape of the output array
    /// - values: values to be broadcast into the array
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice, array};
    /// //  create [5, 4] array filled with 7
    /// let r = Array::full_device::<f32>(&[5, 4], array!(7.0f32), StreamOrDevice::default()).unwrap();
    /// ```
    #[default_device]
    pub fn full_device<T: ArrayElement>(
        shape: &[i32],
        values: impl AsRef<Array>,
        stream: impl AsRef<Stream>,
    ) -> Result<Array> {
        Array::try_from_op(|res| unsafe {
            mlx_sys::mlx_full(
                res,
                shape.as_ptr(),
                shape.len(),
                values.as_ref().c_array,
                T::DTYPE.into(),
                stream.as_ref().as_ptr(),
            )
        })
    }

    /// Create a square identity matrix returning an error if params are invalid.
    ///
    /// # Params
    ///
    /// - n: number of rows and columns in the output
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    /// //  create [10, 10] array with 1's on the diagonal.
    /// let r = Array::identity_device::<f32>(10, StreamOrDevice::default()).unwrap();
    /// ```
    #[default_device]
    pub fn identity_device<T: ArrayElement>(n: i32, stream: impl AsRef<Stream>) -> Result<Array> {
        Array::try_from_op(|res| unsafe {
            mlx_sys::mlx_identity(res, n, T::DTYPE.into(), stream.as_ref().as_ptr())
        })
    }

    /// Generates ranges of numbers.
    ///
    /// Generate numbers in the half-open interval `[start, stop)` in increments of `step`.
    ///
    /// # Params
    ///
    /// - `start`: Starting value which defaults to `0`.
    /// - `stop`: Stopping value.
    /// - `step`: Increment which defaults to `1`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    ///
    /// // Create a 1-D array with values from 0 to 50
    /// let r = Array::arange::<f32, _>(None, 50, None);
    /// ```
    #[default_device]
    pub fn arange_device<T, U>(
        start: impl Into<Option<U>>,
        stop: U,
        step: impl Into<Option<U>>,
        stream: impl AsRef<Stream>,
    ) -> Result<Array>
    where
        T: ArrayElement,
        U: NumCast,
    {
        let start: f64 = start.into().and_then(NumCast::from).unwrap_or(0.0);
        let stop: f64 = NumCast::from(stop).unwrap();
        let step: f64 = step.into().and_then(NumCast::from).unwrap_or(1.0);

        Array::try_from_op(|res| unsafe {
            mlx_sys::mlx_arange(
                res,
                start,
                stop,
                step,
                T::DTYPE.into(),
                stream.as_ref().as_ptr(),
            )
        })
    }

    /// Generate `num` evenly spaced numbers over interval `[start, stop]` returning an error if params are invalid.
    ///
    /// # Params
    ///
    /// - start: start value
    /// - stop: stop value
    /// - count: number of samples -- defaults to 50 if not specified
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    /// // Create a 50 element 1-D array with values from 0 to 50
    /// let r = Array::linspace_device::<f32, _>(0, 50, None, StreamOrDevice::default()).unwrap();
    /// ```
    #[default_device]
    pub fn linspace_device<T, U>(
        start: U,
        stop: U,
        count: impl Into<Option<i32>>,
        stream: impl AsRef<Stream>,
    ) -> Result<Array>
    where
        T: ArrayElement,
        U: NumCast,
    {
        let count = count.into().unwrap_or(50);
        let start_f32 = NumCast::from(start).unwrap();
        let stop_f32 = NumCast::from(stop).unwrap();

        Array::try_from_op(|res| unsafe {
            mlx_sys::mlx_linspace(
                res,
                start_f32,
                stop_f32,
                count,
                T::DTYPE.into(),
                stream.as_ref().as_ptr(),
            )
        })
    }

    /// Repeat an array along a specified axis returning an error if params are invalid.
    ///
    /// # Params
    ///
    /// - array: array to repeat
    /// - count: number of times to repeat
    /// - axis: axis to repeat along
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    /// // repeat a [2, 2] array 4 times along axis 1
    /// let source = Array::from_slice(&[0, 1, 2, 3], &[2, 2]);
    /// let r = Array::repeat_device::<i32>(source, 4, 1, StreamOrDevice::default()).unwrap();
    /// ```
    #[default_device]
    pub fn repeat_device<T: ArrayElement>(
        array: Array,
        count: i32,
        axis: i32,
        stream: impl AsRef<Stream>,
    ) -> Result<Array> {
        Array::try_from_op(|res| unsafe {
            mlx_sys::mlx_repeat(res, array.c_array, count, axis, stream.as_ref().as_ptr())
        })
    }

    /// Repeat a flattened array along axis 0 returning an error if params are invalid.
    ///
    /// # Params
    ///
    /// - array: array to repeat
    /// - count: number of times to repeat
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    /// // repeat a 4 element array 4 times along axis 0
    /// let source = Array::from_slice(&[0, 1, 2, 3], &[2, 2]);
    /// let r = Array::repeat_all_device::<i32>(source, 4, StreamOrDevice::default()).unwrap();
    /// ```
    #[default_device]
    pub fn repeat_all_device<T: ArrayElement>(
        array: Array,
        count: i32,
        stream: impl AsRef<Stream>,
    ) -> Result<Array> {
        Array::try_from_op(|res| unsafe {
            mlx_sys::mlx_repeat_all(res, array.c_array, count, stream.as_ref().as_ptr())
        })
    }

    /// An array with ones at and below the given diagonal and zeros elsewhere.
    ///
    /// # Params
    ///
    /// - n: number of rows in the output
    /// - m: number of columns in the output -- equal to `n` if not specified
    /// - k: index of the diagonal -- defaults to 0 if not specified
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::{Array, StreamOrDevice};
    /// // [5, 5] array with the lower triangle filled with 1s
    /// let r = Array::tri_device::<f32>(5, None, None, StreamOrDevice::default());
    /// ```
    #[default_device]
    pub fn tri_device<T: ArrayElement>(
        n: i32,
        m: Option<i32>,
        k: Option<i32>,
        stream: impl AsRef<Stream>,
    ) -> Result<Array> {
        Array::try_from_op(|res| unsafe {
            mlx_sys::mlx_tri(
                res,
                n,
                m.unwrap_or(n),
                k.unwrap_or(0),
                T::DTYPE.into(),
                stream.as_ref().as_ptr(),
            )
        })
    }
}

/// See [`Array::zeros`]
#[default_device]
pub fn zeros_device<T: ArrayElement>(shape: &[i32], stream: impl AsRef<Stream>) -> Result<Array> {
    Array::zeros_device::<T>(shape, stream)
}

/// An array of zeros like the input.
#[default_device]
pub fn zeros_like_device(input: impl AsRef<Array>, stream: impl AsRef<Stream>) -> Result<Array> {
    let a = input.as_ref();
    let shape = a.shape();
    let dtype = a.dtype();
    zeros_dtype_device(shape, dtype, stream)
}

/// Similar to [`Array::zeros`] but with a specified dtype.
#[default_device]
pub fn zeros_dtype_device(
    shape: &[i32],
    dtype: Dtype,
    stream: impl AsRef<Stream>,
) -> Result<Array> {
    Array::try_from_op(|res| unsafe {
        mlx_sys::mlx_zeros(
            res,
            shape.as_ptr(),
            shape.len(),
            dtype.into(),
            stream.as_ref().as_ptr(),
        )
    })
}

/// See [`Array::ones`]
#[default_device]
pub fn ones_device<T: ArrayElement>(shape: &[i32], stream: impl AsRef<Stream>) -> Result<Array> {
    Array::ones_device::<T>(shape, stream)
}

/// An array of ones like the input.
#[default_device]
pub fn ones_like_device(input: impl AsRef<Array>, stream: impl AsRef<Stream>) -> Result<Array> {
    let a = input.as_ref();
    let shape = a.shape();
    let dtype = a.dtype();
    ones_dtype_device(shape, dtype, stream)
}

/// Similar to [`Array::ones`] but with a specified dtype.
pub fn ones_dtype_device(shape: &[i32], dtype: Dtype, stream: impl AsRef<Stream>) -> Result<Array> {
    Array::try_from_op(|res| unsafe {
        mlx_sys::mlx_ones(
            res,
            shape.as_ptr(),
            shape.len(),
            dtype.into(),
            stream.as_ref().as_ptr(),
        )
    })
}

/// See [`Array::eye`]
#[default_device]
pub fn eye_device<T: ArrayElement>(
    n: i32,
    m: Option<i32>,
    k: Option<i32>,
    stream: impl AsRef<Stream>,
) -> Result<Array> {
    Array::eye_device::<T>(n, m, k, stream)
}

/// See [`Array::full`]
#[default_device]
pub fn full_device<T: ArrayElement>(
    shape: &[i32],
    values: impl AsRef<Array>,
    stream: impl AsRef<Stream>,
) -> Result<Array> {
    Array::full_device::<T>(shape, values, stream)
}

/// See [`Array::identity`]
#[default_device]
pub fn identity_device<T: ArrayElement>(n: i32, stream: impl AsRef<Stream>) -> Result<Array> {
    Array::identity_device::<T>(n, stream)
}

/// See [`Array::arange`]
#[default_device]
pub fn arange_device<T, U>(
    start: impl Into<Option<U>>,
    stop: U,
    step: impl Into<Option<U>>,
    stream: impl AsRef<Stream>,
) -> Result<Array>
where
    T: ArrayElement,
    U: NumCast,
{
    Array::arange_device::<T, U>(start, stop, step, stream)
}

/// See [`Array::linspace`]
#[default_device]
pub fn linspace_device<T, U>(
    start: U,
    stop: U,
    count: impl Into<Option<i32>>,
    stream: impl AsRef<Stream>,
) -> Result<Array>
where
    T: ArrayElement,
    U: NumCast,
{
    Array::linspace_device::<T, U>(start, stop, count, stream)
}

/// See [`Array::repeat`]
#[default_device]
pub fn repeat_device<T: ArrayElement>(
    array: Array,
    count: i32,
    axis: i32,
    stream: impl AsRef<Stream>,
) -> Result<Array> {
    Array::repeat_device::<T>(array, count, axis, stream)
}

/// See [`Array::repeat_all`]
#[default_device]
pub fn repeat_all_device<T: ArrayElement>(
    array: Array,
    count: i32,
    stream: impl AsRef<Stream>,
) -> Result<Array> {
    Array::repeat_all_device::<T>(array, count, stream)
}

/// See [`Array::tri`]
#[default_device]
pub fn tri_device<T: ArrayElement>(
    n: i32,
    m: Option<i32>,
    k: Option<i32>,
    stream: impl AsRef<Stream>,
) -> Result<Array> {
    Array::tri_device::<T>(n, m, k, stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{array, dtype::Dtype};
    use half::f16;

    #[test]
    fn test_zeros() {
        let array = Array::zeros::<f32>(&[2, 3]).unwrap();
        assert_eq!(array.shape(), &[2, 3]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        assert_eq!(data, &[0.0; 6]);
    }

    #[test]
    fn test_zeros_try() {
        let array = Array::zeros::<f32>(&[2, 3]);
        assert!(array.is_ok());

        let array = Array::zeros::<f32>(&[-1, 3]);
        assert!(array.is_err());
    }

    #[test]
    fn test_ones() {
        let array = Array::ones::<f16>(&[2, 3]).unwrap();
        assert_eq!(array.shape(), &[2, 3]);
        assert_eq!(array.dtype(), Dtype::Float16);

        let data: &[f16] = array.as_slice();
        assert_eq!(data, &[f16::from_f32(1.0); 6]);
    }

    #[test]
    fn test_eye() {
        let array = Array::eye::<f32>(3, None, None).unwrap();
        assert_eq!(array.shape(), &[3, 3]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        assert_eq!(data, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_full_scalar() {
        let array = Array::full::<f32>(&[2, 3], array!(7f32)).unwrap();
        assert_eq!(array.shape(), &[2, 3]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        assert_eq!(data, &[7.0; 6]);
    }

    #[test]
    fn test_full_array() {
        let source = Array::zeros_device::<f32>(&[1, 3], StreamOrDevice::cpu()).unwrap();
        let array = Array::full::<f32>(&[2, 3], source).unwrap();
        assert_eq!(array.shape(), &[2, 3]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        float_eq::float_eq!(*data, [0.0; 6], abs <= [1e-6; 6]);
    }

    #[test]
    fn test_full_try() {
        let source = Array::zeros_device::<f32>(&[1, 3], StreamOrDevice::default()).unwrap();
        let array = Array::full::<f32>(&[2, 3], source);
        assert!(array.is_ok());

        let source = Array::zeros_device::<f32>(&[1, 3], StreamOrDevice::default()).unwrap();
        let array = Array::full::<f32>(&[-1, 3], source);
        assert!(array.is_err());
    }

    #[test]
    fn test_identity() {
        let array = Array::identity::<f32>(3).unwrap();
        assert_eq!(array.shape(), &[3, 3]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        assert_eq!(data, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_arange() {
        let array = Array::arange::<f32, _>(None, 50, None).unwrap();
        assert_eq!(array.shape(), &[50]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        let expected: Vec<f32> = (0..50).map(|x| x as f32).collect();
        assert_eq!(data, expected.as_slice());

        let array = Array::arange::<i32, _>(0, 50, None).unwrap();
        assert_eq!(array.shape(), &[50]);
        assert_eq!(array.dtype(), Dtype::Int32);

        let data: &[i32] = array.as_slice();
        let expected: Vec<i32> = (0..50).collect();
        assert_eq!(data, expected.as_slice());

        let result = Array::arange::<bool, _>(None, 50, None);
        assert!(result.is_err());

        let result = Array::arange::<f32, _>(f64::NEG_INFINITY, 50.0, None);
        assert!(result.is_err());

        let result = Array::arange::<f32, _>(0.0, f64::INFINITY, None);
        assert!(result.is_err());

        let result = Array::arange::<f32, _>(0.0, 50.0, f32::NAN);
        assert!(result.is_err());

        let result = Array::arange::<f32, _>(f32::NAN, 50.0, None);
        assert!(result.is_err());

        let result = Array::arange::<f32, _>(0.0, f32::NAN, None);
        assert!(result.is_err());

        let result = Array::arange::<f32, _>(0, i32::MAX as i64 + 1, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_linspace_int() {
        let array = Array::linspace::<f32, _>(0, 50, None).unwrap();
        assert_eq!(array.shape(), &[50]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        let expected: Vec<f32> = (0..50).map(|x| x as f32 * (50.0 / 49.0)).collect();
        assert_eq!(data, expected.as_slice());
    }

    #[test]
    fn test_linspace_float() {
        let array = Array::linspace::<f32, _>(0., 50., None).unwrap();
        assert_eq!(array.shape(), &[50]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        let expected: Vec<f32> = (0..50).map(|x| x as f32 * (50.0 / 49.0)).collect();
        assert_eq!(data, expected.as_slice());
    }

    #[test]
    fn test_linspace_try() {
        let array = Array::linspace::<f32, _>(0, 50, None);
        assert!(array.is_ok());

        let array = Array::linspace::<f32, _>(0, 50, Some(-1));
        assert!(array.is_err());
    }

    #[test]
    fn test_repeat() {
        let source = Array::from_slice(&[0, 1, 2, 3], &[2, 2]);
        let array = Array::repeat::<i32>(source, 4, 1).unwrap();
        assert_eq!(array.shape(), &[2, 8]);
        assert_eq!(array.dtype(), Dtype::Int32);

        let data: &[i32] = array.as_slice();
        assert_eq!(data, [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3]);
    }

    #[test]
    fn test_repeat_try() {
        let source = Array::from_slice(&[0, 1, 2, 3], &[2, 2]);
        let array = Array::repeat::<i32>(source, 4, 1);
        assert!(array.is_ok());

        let source = Array::from_slice(&[0, 1, 2, 3], &[2, 2]);
        let array = Array::repeat::<i32>(source, -1, 1);
        assert!(array.is_err());
    }

    #[test]
    fn test_repeat_all() {
        let source = Array::from_slice(&[0, 1, 2, 3], &[2, 2]);
        let array = Array::repeat_all::<i32>(source, 4).unwrap();
        assert_eq!(array.shape(), &[16]);
        assert_eq!(array.dtype(), Dtype::Int32);

        let data: &[i32] = array.as_slice();
        assert_eq!(data, [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3]);
    }

    #[test]
    fn test_repeat_all_try() {
        let source = Array::from_slice(&[0, 1, 2, 3], &[2, 2]);
        let array = Array::repeat_all::<i32>(source, 4);
        assert!(array.is_ok());

        let source = Array::from_slice(&[0, 1, 2, 3], &[2, 2]);
        let array = Array::repeat_all::<i32>(source, -1);
        assert!(array.is_err());
    }

    #[test]
    fn test_tri() {
        let array = Array::tri::<f32>(3, None, None).unwrap();
        assert_eq!(array.shape(), &[3, 3]);
        assert_eq!(array.dtype(), Dtype::Float32);

        let data: &[f32] = array.as_slice();
        assert_eq!(data, &[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0]);
    }
}
