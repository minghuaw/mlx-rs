use crate::{
    dtype::Dtype,
    error::{
        get_and_clear_last_mlx_error, is_mlx_error_handler_set, setup_mlx_error_handler,
        AsSliceError, Exception, ItemError,
    },
    sealed::Sealed,
    StreamOrDevice,
};
use mlx_sys::mlx_array;
use num_complex::Complex;
use std::ffi::c_void;

mod element;
mod operators;

pub use element::ArrayElement;

// Not using Complex64 because `num_complex::Complex64` is actually Complex<f64>
#[allow(non_camel_case_types)]
pub type complex64 = Complex<f32>;

#[repr(transparent)]
pub struct Array {
    pub(crate) c_array: mlx_array,
}

impl Sealed for Array {}

impl<'a> Sealed for &'a Array {}

impl std::fmt::Debug for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = crate::utils::mlx_describe(self.c_array as *mut c_void)
            .unwrap_or_else(|| "Array".to_string());
        write!(f, "{:?}", description)
    }
}

// TODO: Clone should probably NOT be implemented because the underlying pointer is atomically
// reference counted but not guarded by a mutex.

impl Array {
    /// Clone the array by copying the data.
    pub(crate) fn clone(&self) -> Self {
        unsafe {
            let dtype = self.dtype();
            let shape = self.shape();
            let data = match dtype {
                Dtype::Bool => mlx_sys::mlx_array_data_bool(self.c_array) as *const c_void,
                Dtype::Uint8 => mlx_sys::mlx_array_data_uint8(self.c_array) as *const c_void,
                Dtype::Uint16 => mlx_sys::mlx_array_data_uint16(self.c_array) as *const c_void,
                Dtype::Uint32 => mlx_sys::mlx_array_data_uint32(self.c_array) as *const c_void,
                Dtype::Uint64 => mlx_sys::mlx_array_data_uint64(self.c_array) as *const c_void,
                Dtype::Int8 => mlx_sys::mlx_array_data_int8(self.c_array) as *const c_void,
                Dtype::Int16 => mlx_sys::mlx_array_data_int16(self.c_array) as *const c_void,
                Dtype::Int32 => mlx_sys::mlx_array_data_int32(self.c_array) as *const c_void,
                Dtype::Int64 => mlx_sys::mlx_array_data_int64(self.c_array) as *const c_void,
                Dtype::Float16 => mlx_sys::mlx_array_data_float16(self.c_array) as *const c_void,
                Dtype::Float32 => mlx_sys::mlx_array_data_float32(self.c_array) as *const c_void,
                Dtype::Bfloat16 => mlx_sys::mlx_array_data_bfloat16(self.c_array) as *const c_void,
                Dtype::Complex64 => {
                    mlx_sys::mlx_array_data_complex64(self.c_array) as *const c_void
                }
            };

            let new_c_array = mlx_sys::mlx_array_from_data(
                data,
                shape.as_ptr(),
                shape.len() as i32,
                dtype.into(),
            );

            Array::from_ptr(new_c_array)
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        // TODO: check memory leak with some tool?

        // Decrease the reference count
        unsafe { mlx_sys::mlx_free(self.c_array as *mut c_void) };
    }
}

unsafe impl Send for Array {}

impl PartialEq for Array {
    /// Array equality check.
    ///
    /// Compare two arrays for equality. Returns `true` iff the arrays have
    /// the same shape and their values are equal. The arrays need not have
    /// the same type to be considered equal.
    ///
    /// If you're looking for element-wise equality, use the [Array::eq()] method.
    fn eq(&self, other: &Self) -> bool {
        self.array_eq(other, None).item()
    }
}

impl Array {
    /// Create a new array from an existing mlx_array pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure the reference count of the array is properly incremented with
    /// `mlx_sys::mlx_retain`.
    pub unsafe fn from_ptr(c_array: mlx_array) -> Array {
        Self { c_array }
    }

    // TODO: should this be unsafe?
    pub fn as_ptr(&self) -> mlx_array {
        self.c_array
    }

    /// New array from a bool scalar.
    pub fn from_bool(val: bool) -> Array {
        let c_array = unsafe { mlx_sys::mlx_array_from_bool(val) };
        Array { c_array }
    }

    /// New array from an int scalar.
    pub fn from_int(val: i32) -> Array {
        let c_array = unsafe { mlx_sys::mlx_array_from_int(val) };
        Array { c_array }
    }

    /// New array from a float scalar.
    pub fn from_float(val: f32) -> Array {
        let c_array = unsafe { mlx_sys::mlx_array_from_float(val) };
        Array { c_array }
    }

    /// New array from a complex scalar.
    pub fn from_complex(val: complex64) -> Array {
        let c_array = unsafe { mlx_sys::mlx_array_from_complex(val.re, val.im) };
        Array { c_array }
    }

    /// New array from existing buffer.
    ///
    /// # Parameters
    ///
    /// - `data`: A buffer which will be copied.
    /// - `shape`: Shape of the array.
    ///
    /// # Panic
    ///
    /// - Panics if the product of the shape is not equal to the length of the data.
    /// - Panics if the shape is too large.
    pub fn from_slice<T: ArrayElement>(data: &[T], shape: &[i32]) -> Self {
        let dim = if shape.len() > i32::MAX as usize {
            panic!("Shape is too large")
        } else {
            shape.len() as i32
        };

        // Validate data size and shape
        assert_eq!(data.len(), shape.iter().product::<i32>() as usize);

        let c_array = unsafe {
            mlx_sys::mlx_array_from_data(
                data.as_ptr() as *const c_void,
                shape.as_ptr(),
                dim,
                T::DTYPE.into(),
            )
        };

        Array { c_array }
    }

    /// New array from an iterator.
    ///
    /// This is a convenience method that is equivalent to
    ///
    /// ```rust, ignore
    /// let data: Vec<T> = iter.collect();
    /// Array::from_slice(&data, shape)
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::Array;
    ///
    /// let data = vec![1i32, 2, 3, 4, 5];
    /// let mut array = Array::from_iter(data.clone(), &[5]);
    /// assert_eq!(array.as_slice::<i32>(), &data[..]);
    /// ```
    pub fn from_iter<T: ArrayElement, I: IntoIterator<Item = T>>(iter: I, shape: &[i32]) -> Self {
        let data: Vec<T> = iter.into_iter().collect();
        Self::from_slice(&data, shape)
    }

    /// The size of the array’s datatype in bytes.
    pub fn item_size(&self) -> usize {
        unsafe { mlx_sys::mlx_array_itemsize(self.c_array) }
    }

    /// Number of elements in the array.
    pub fn size(&self) -> usize {
        unsafe { mlx_sys::mlx_array_size(self.c_array) }
    }

    /// The strides of the array.
    pub fn strides(&self) -> &[usize] {
        let ndim = self.ndim();
        if ndim == 0 {
            // The data pointer may be null which would panic even if len is 0
            return &[];
        }

        unsafe {
            let data = mlx_sys::mlx_array_strides(self.c_array);
            std::slice::from_raw_parts(data, ndim)
        }
    }

    /// The number of bytes in the array.
    pub fn nbytes(&self) -> usize {
        unsafe { mlx_sys::mlx_array_nbytes(self.c_array) }
    }

    /// The array’s dimension.
    pub fn ndim(&self) -> usize {
        unsafe { mlx_sys::mlx_array_ndim(self.c_array) }
    }

    /// The shape of the array.
    ///
    /// Returns: a pointer to the sizes of each dimension.
    pub fn shape(&self) -> &[i32] {
        let ndim = self.ndim();
        if ndim == 0 {
            // The data pointer may be null which would panic even if len is 0
            return &[];
        }

        unsafe {
            let data = mlx_sys::mlx_array_shape(self.c_array);
            std::slice::from_raw_parts(data, ndim)
        }
    }

    /// The shape of the array in a particular dimension.
    ///
    /// # Panic
    ///
    /// - Panics if the array is scalar.
    /// - Panics if `dim` is negative and `dim + ndim` overflows
    /// - Panics if the dimension is out of bounds.
    pub fn dim(&self, dim: i32) -> i32 {
        let dim = if dim.is_negative() {
            (self.ndim() as i32).checked_add(dim).unwrap()
        } else {
            dim
        };

        // This will panic on a scalar array
        unsafe { mlx_sys::mlx_array_dim(self.c_array, dim) }
    }

    /// The array element type.
    pub fn dtype(&self) -> Dtype {
        let dtype = unsafe { mlx_sys::mlx_array_get_dtype(self.c_array) };
        Dtype::try_from(dtype).unwrap()
    }

    // TODO: document that mlx is lazy
    /// Evaluate the array.
    pub fn eval(&mut self) -> Result<(), Exception> {
        if !is_mlx_error_handler_set() {
            setup_mlx_error_handler();
        }

        unsafe { mlx_sys::mlx_array_eval(self.c_array) };

        get_and_clear_last_mlx_error().map_or(Ok(()), Err)
    }

    /// Access the value of a scalar array.
    /// If `T` does not match the array's `dtype` this will convert the type first.
    ///
    /// _Note: This will evaluate the array._
    pub fn item<T: ArrayElement>(&mut self) -> T {
        self.try_item().unwrap()
    }

    /// Access the value of a scalar array without validating the shape.
    /// If `T` does not match the array's `dtype` this will convert the type first.
    ///
    /// _Note: This will evaluate the array._
    ///
    /// # Safety
    ///
    /// This is unsafe because the array is not checked for being a scalar.
    pub fn item_unchecked<T: ArrayElement>(&mut self) -> T {
        // Evaluate the array, so we have content to work with in the conversion
        self.eval().unwrap();

        if self.dtype() != T::DTYPE {
            let new_array_ctx = unsafe {
                mlx_sys::mlx_astype(
                    self.c_array,
                    T::DTYPE.into(),
                    StreamOrDevice::default().as_ptr(),
                )
            };
            let mut new_array = unsafe { Array::from_ptr(new_array_ctx) };
            new_array.eval().unwrap();

            return T::array_item(&new_array);
        }

        T::array_item(self)
    }

    /// Access the value of a scalar array returning an error if the array is not a scalar.
    /// If `T` does not match the array's `dtype` this will convert the type first.
    ///
    /// _Note: This will evaluate the array._
    pub fn try_item<T: ArrayElement>(&mut self) -> Result<T, ItemError> {
        if self.size() != 1 {
            return Err(ItemError::NotScalar);
        }

        // Evaluate the array, so we have content to work with in the conversion
        self.eval()?;

        if self.dtype() != T::DTYPE {
            let new_array_ctx = unsafe {
                mlx_sys::mlx_astype(
                    self.c_array,
                    T::DTYPE.into(),
                    StreamOrDevice::default().as_ptr(),
                )
            };
            let mut new_array = unsafe { Array::from_ptr(new_array_ctx) };
            new_array.eval()?;

            return Ok(T::array_item(&new_array));
        }

        Ok(T::array_item(self))
    }

    /// Returns a slice of the array data without validating the dtype.
    /// This method requires a mutable reference (`&mut self`) because it evaluates the array.
    ///
    /// # Safety
    ///
    /// This is unsafe because the underlying data ptr is not checked for null or if the desired
    /// dtype matches the actual dtype of the array.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::Array;
    ///
    /// let data = [1i32, 2, 3, 4, 5];
    /// let mut array = Array::from_slice(&data[..], &[5]);
    ///
    /// unsafe {
    ///    let slice = array.as_slice_unchecked::<i32>();
    ///    assert_eq!(slice, &[1, 2, 3, 4, 5]);
    /// }
    /// ```
    pub unsafe fn as_slice_unchecked<T: ArrayElement>(&mut self) -> &[T] {
        self.eval().unwrap();

        unsafe {
            let data = T::array_data(self);
            let size = self.size();
            std::slice::from_raw_parts(data, size)
        }
    }

    /// Returns a slice of the array data returning an error if the dtype does not match the actual dtype.
    /// This method requires a mutable reference (`&mut self`) because it evaluates the array.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::Array;
    ///
    /// let data = [1i32, 2, 3, 4, 5];
    /// let mut array = Array::from_slice(&data[..], &[5]);
    ///
    /// let slice = array.try_as_slice::<i32>();
    /// assert_eq!(slice, Ok(&data[..]));
    /// ```
    pub fn try_as_slice<T: ArrayElement>(&mut self) -> Result<&[T], AsSliceError> {
        if self.dtype() != T::DTYPE {
            return Err(AsSliceError::DtypeMismatch {
                expecting: T::DTYPE,
                found: self.dtype(),
            });
        }

        self.eval()?;

        unsafe {
            let size = self.size();
            let data = T::array_data(self);
            if data.is_null() || size == 0 {
                return Err(AsSliceError::Null);
            }

            Ok(std::slice::from_raw_parts(data, size))
        }
    }

    /// Returns a slice of the array data.
    /// This method requires a mutable reference (`&mut self`) because it evaluates the array.
    ///
    /// # Panics
    ///
    /// Panics if the array is not evaluated or if the desired dtype does not match the actual dtype
    ///
    /// # Example
    ///
    /// ```rust
    /// use mlx_rs::Array;
    ///
    /// let data = [1i32, 2, 3, 4, 5];
    /// let mut array = Array::from_slice(&data[..], &[5]);
    ///
    /// let slice = array.as_slice::<i32>();
    /// assert_eq!(slice, &data[..]);
    /// ```
    pub fn as_slice<T: ArrayElement>(&mut self) -> &[T] {
        self.try_as_slice().unwrap()
    }
}

impl From<bool> for Array {
    fn from(val: bool) -> Self {
        Array::from_bool(val)
    }
}

impl From<i32> for Array {
    fn from(val: i32) -> Self {
        Array::from_int(val)
    }
}

impl From<f32> for Array {
    fn from(val: f32) -> Self {
        Array::from_float(val)
    }
}

impl AsRef<Array> for Array {
    fn as_ref(&self) -> &Array {
        self
    }
}

/// A helper trait to construct `Array` from nested arrays or slices.
///
/// Given that this is not intended for use other than the macro [`array!`], we added this trait
/// instead of directly implementing `From` for `Array` to avoid conflicts with other `From`
/// implementations.
///
/// Beware that this is subject to change in the future should we find a better way to implement
/// the macro without creating conflicts.
pub trait FromNested<T> {
    fn from_nested(data: T) -> Array;
}

impl<T: ArrayElement> FromNested<&[T]> for Array {
    fn from_nested(data: &[T]) -> Self {
        Array::from_slice(data, &[data.len() as i32])
    }
}

impl<T: ArrayElement, const N: usize> FromNested<[T; N]> for Array {
    fn from_nested(data: [T; N]) -> Self {
        Array::from_slice(&data, &[N as i32])
    }
}

impl<T: ArrayElement, const N: usize> FromNested<&[T; N]> for Array {
    fn from_nested(data: &[T; N]) -> Self {
        Array::from_slice(data, &[N as i32])
    }
}

impl<T: ArrayElement + Copy> FromNested<&[&[T]]> for Array {
    fn from_nested(data: &[&[T]]) -> Self {
        // check that all rows have the same length
        let row_len = data[0].len();
        assert!(
            data.iter().all(|row| row.len() == row_len),
            "Rows must have the same length"
        );

        let shape = [data.len() as i32, row_len as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter())
            .copied()
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize> FromNested<[&[T]; N]> for Array {
    fn from_nested(data: [&[T]; N]) -> Self {
        // check that all rows have the same length
        let row_len = data[0].len();
        assert!(
            data.iter().all(|row| row.len() == row_len),
            "Rows must have the same length"
        );

        let shape = [N as i32, row_len as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter())
            .copied()
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize> FromNested<&[[T; N]]> for Array {
    fn from_nested(data: &[[T; N]]) -> Self {
        let shape = [data.len() as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().copied())
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize> FromNested<&[&[T; N]]> for Array {
    fn from_nested(data: &[&[T; N]]) -> Self {
        let shape = [data.len() as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().copied())
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize> FromNested<[[T; N]; M]> for Array {
    fn from_nested(data: [[T; N]; M]) -> Self {
        let shape = [M as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().copied())
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize> FromNested<&[[T; N]; M]> for Array {
    fn from_nested(data: &[[T; N]; M]) -> Self {
        let shape = [M as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().copied())
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize> FromNested<&[&[T; N]; M]> for Array {
    fn from_nested(data: &[&[T; N]; M]) -> Self {
        let shape = [M as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().copied())
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy> FromNested<&[&[&[T]]]> for Array {
    fn from_nested(data: &[&[&[T]]]) -> Self {
        // check that 2nd dimension has the same length
        let len_2d = data[0].len();
        assert!(
            data.iter().all(|x| x.len() == len_2d),
            "2nd dimension must have the same length"
        );

        // check that 3rd dimension has the same length
        let len_3d = data[0][0].len();
        assert!(
            data.iter().all(|x| x.iter().all(|y| y.len() == len_3d)),
            "3rd dimension must have the same length"
        );

        let shape = [data.len() as i32, len_2d as i32, len_3d as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize> FromNested<[&[&[T]]; N]> for Array {
    fn from_nested(data: [&[&[T]]; N]) -> Self {
        // check that 2nd dimension has the same length
        let len_2d = data[0].len();
        assert!(
            data.iter().all(|x| x.len() == len_2d),
            "2nd dimension must have the same length"
        );

        // check that 3rd dimension has the same length
        let len_3d = data[0][0].len();
        assert!(
            data.iter().all(|x| x.iter().all(|y| y.len() == len_3d)),
            "3rd dimension must have the same length"
        );

        let shape = [N as i32, len_2d as i32, len_3d as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize> FromNested<&[[&[T]; N]]> for Array {
    fn from_nested(data: &[[&[T]; N]]) -> Self {
        // check that 3rd dimension has the same length
        let len_3d = data[0][0].len();
        assert!(
            data.iter().all(|x| x.iter().all(|y| y.len() == len_3d)),
            "3rd dimension must have the same length"
        );

        let shape = [data.len() as i32, N as i32, len_3d as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize> FromNested<&[&[[T; N]]]> for Array {
    fn from_nested(data: &[&[[T; N]]]) -> Self {
        // check that 2nd dimension has the same length
        let len_2d = data[0].len();
        assert!(
            data.iter().all(|x| x.len() == len_2d),
            "2nd dimension must have the same length"
        );

        let shape = [data.len() as i32, len_2d as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize> FromNested<[[&[T]; N]; M]> for Array {
    fn from_nested(data: [[&[T]; N]; M]) -> Self {
        // check that 3rd dimension has the same length
        let len_3d = data[0][0].len();
        assert!(
            data.iter().all(|x| x.iter().all(|y| y.len() == len_3d)),
            "3rd dimension must have the same length"
        );

        let shape = [M as i32, N as i32, len_3d as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize> FromNested<&[[&[T]; N]; M]> for Array {
    fn from_nested(data: &[[&[T]; N]; M]) -> Self {
        // check that 3rd dimension has the same length
        let len_3d = data[0][0].len();
        assert!(
            data.iter().all(|x| x.iter().all(|y| y.len() == len_3d)),
            "3rd dimension must have the same length"
        );

        let shape = [M as i32, N as i32, len_3d as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize> FromNested<&[&[[T; N]]; M]> for Array {
    fn from_nested(data: &[&[[T; N]]; M]) -> Self {
        // check that 2nd dimension has the same length
        let len_2d = data[0].len();
        assert!(
            data.iter().all(|x| x.len() == len_2d),
            "2nd dimension must have the same length"
        );

        let shape = [M as i32, len_2d as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize, const O: usize>
    FromNested<[[[T; N]; M]; O]> for Array
{
    fn from_nested(data: [[[T; N]; M]; O]) -> Self {
        let shape = [O as i32, M as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize, const O: usize>
    FromNested<&[[[T; N]; M]; O]> for Array
{
    fn from_nested(data: &[[[T; N]; M]; O]) -> Self {
        let shape = [O as i32, M as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize, const O: usize>
    FromNested<&[&[[T; N]; M]; O]> for Array
{
    fn from_nested(data: &[&[[T; N]; M]; O]) -> Self {
        let shape = [O as i32, M as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize, const O: usize>
    FromNested<&[[&[T; N]; M]; O]> for Array
{
    fn from_nested(data: &[[&[T; N]; M]; O]) -> Self {
        let shape = [O as i32, M as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

impl<T: ArrayElement + Copy, const N: usize, const M: usize, const O: usize>
    FromNested<&[&[&[T; N]; M]; O]> for Array
{
    fn from_nested(data: &[&[&[T; N]; M]; O]) -> Self {
        let shape = [O as i32, M as i32, N as i32];
        let data = data
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().copied()))
            .collect::<Vec<T>>();
        Array::from_slice(&data, &shape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_scalar_array_from_bool() {
        let mut array = Array::from_bool(true);
        assert!(array.item::<bool>());
        assert_eq!(array.item_size(), 1);
        assert_eq!(array.size(), 1);
        assert!(array.strides().is_empty());
        assert_eq!(array.nbytes(), 1);
        assert_eq!(array.ndim(), 0);
        assert!(array.shape().is_empty());
        assert_eq!(array.dtype(), Dtype::Bool);
    }

    #[test]
    fn new_scalar_array_from_int() {
        let mut array = Array::from_int(42);
        assert_eq!(array.item::<i32>(), 42);
        assert_eq!(array.item_size(), 4);
        assert_eq!(array.size(), 1);
        assert!(array.strides().is_empty());
        assert_eq!(array.nbytes(), 4);
        assert_eq!(array.ndim(), 0);
        assert!(array.shape().is_empty());
        assert_eq!(array.dtype(), Dtype::Int32);
    }

    #[test]
    fn new_scalar_array_from_float() {
        let mut array = Array::from_float(3.14);
        assert_eq!(array.item::<f32>(), 3.14);
        assert_eq!(array.item_size(), 4);
        assert_eq!(array.size(), 1);
        assert!(array.strides().is_empty());
        assert_eq!(array.nbytes(), 4);
        assert_eq!(array.ndim(), 0);
        assert!(array.shape().is_empty());
        assert_eq!(array.dtype(), Dtype::Float32);
    }

    #[test]
    fn new_scalar_array_from_complex() {
        let val = complex64::new(1.0, 2.0);
        let mut array = Array::from_complex(val);
        assert_eq!(array.item::<complex64>(), val);
        assert_eq!(array.item_size(), 8);
        assert_eq!(array.size(), 1);
        assert!(array.strides().is_empty());
        assert_eq!(array.nbytes(), 8);
        assert_eq!(array.ndim(), 0);
        assert!(array.shape().is_empty());
        assert_eq!(array.dtype(), Dtype::Complex64);
    }

    #[test]
    fn new_array_from_single_element_slice() {
        let data = [1i32];
        let mut array = Array::from_slice(&data, &[1]);
        assert_eq!(array.as_slice::<i32>(), &data[..]);
        assert_eq!(array.item::<i32>(), 1);
        assert_eq!(array.item_size(), 4);
        assert_eq!(array.size(), 1);
        assert_eq!(array.strides(), &[1]);
        assert_eq!(array.nbytes(), 4);
        assert_eq!(array.ndim(), 1);
        assert_eq!(array.dim(0), 1);
        assert_eq!(array.shape(), &[1]);
        assert_eq!(array.dtype(), Dtype::Int32);
    }

    #[test]
    fn new_array_from_multi_element_slice() {
        let data = [1i32, 2, 3, 4, 5];
        let mut array = Array::from_slice(&data, &[5]);
        assert_eq!(array.as_slice::<i32>(), &data[..]);
        assert_eq!(array.item_size(), 4);
        assert_eq!(array.size(), 5);
        assert_eq!(array.strides(), &[1]);
        assert_eq!(array.nbytes(), 20);
        assert_eq!(array.ndim(), 1);
        assert_eq!(array.dim(0), 5);
        assert_eq!(array.shape(), &[5]);
        assert_eq!(array.dtype(), Dtype::Int32);
    }

    #[test]
    fn new_2d_array_from_slice() {
        let data = [1i32, 2, 3, 4, 5, 6];
        let mut array = Array::from_slice(&data, &[2, 3]);
        assert_eq!(array.as_slice::<i32>(), &data[..]);
        assert_eq!(array.item_size(), 4);
        assert_eq!(array.size(), 6);
        assert_eq!(array.strides(), &[3, 1]);
        assert_eq!(array.nbytes(), 24);
        assert_eq!(array.ndim(), 2);
        assert_eq!(array.dim(0), 2);
        assert_eq!(array.dim(1), 3);
        assert_eq!(array.dim(-1), 3); // negative index
        assert_eq!(array.dim(-2), 2); // negative index
        assert_eq!(array.shape(), &[2, 3]);
        assert_eq!(array.dtype(), Dtype::Int32);
    }

    #[test]
    fn cloned_array_has_different_ptr() {
        let data = [1i32, 2, 3, 4, 5];
        let mut orig = Array::from_slice(&data, &[5]);
        let mut clone = orig.clone();

        // Data should be the same
        assert_eq!(orig.as_slice::<i32>(), clone.as_slice::<i32>());

        // Addr of `mlx_array` should be different
        assert_ne!(orig.as_ptr(), clone.as_ptr());

        // Addr of data should be different
        assert_ne!(
            orig.as_slice::<i32>().as_ptr(),
            clone.as_slice::<i32>().as_ptr()
        );
    }

    #[test]
    fn test_array_eq() {
        let data = [1i32, 2, 3, 4, 5];
        let array1 = Array::from_slice(&data, &[5]);
        let array2 = Array::from_slice(&data, &[5]);
        let array3 = Array::from_slice(&[1i32, 2, 3, 4, 6], &[5]);

        assert_eq!(&array1, &array2);
        assert_ne!(&array1, &array3);
    }

    #[test]
    fn test_array_item_non_scalar() {
        let data = [1i32, 2, 3, 4, 5];
        let mut array = Array::from_slice(&data, &[5]);
        assert!(array.try_item::<i32>().is_err());
    }

    #[test]
    fn test_item_type_conversion() {
        let mut array = Array::from_float(1.0);
        assert_eq!(array.item::<i32>(), 1);
        assert_eq!(array.item::<complex64>(), complex64::new(1.0, 0.0));
        assert_eq!(array.item::<u8>(), 1);

        assert_eq!(array.as_slice::<f32>(), &[1.0]);
    }
}
