use cxx::UniquePtr;

use crate::{Optional, array::ffi::array};

// pub type OptionalArray = Optional<UniquePtr<array>>;

use crate::ops::OptionalArray;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("mlx/ops.h");
        include!("mlx/random.h");

        include!("mlx-cxx/random.hpp");

        // TODO: add constructor binding for KeySequence
        #[namespace = "mlx::core::random"]
        type KeySequence;

        #[namespace = "mlx::core"]
        type array = crate::array::ffi::array;

        #[namespace = "mlx_cxx"]
        type OptionalArray = crate::random::OptionalArray;

        #[namespace = "mlx_cxx"]
        type StreamOrDevice = crate::StreamOrDevice;

        #[namespace = "mlx::core"]
        type Dtype = crate::dtype::ffi::Dtype;

        #[namespace = "mlx::core"]
        type float16_t = crate::types::float16::float16_t;

        #[namespace = "mlx::core"]
        type bfloat16_t = crate::types::bfloat16::bfloat16_t;

        #[namespace = "mlx::core"]
        type complex64_t = crate::types::complex64::complex64_t;

        #[namespace = "mlx_cxx"]
        fn key_sequence(seed: u64) -> UniquePtr<KeySequence>;

        #[namespace = "mlx_cxx"]
        fn default_key_sequence() -> &'static KeySequence;

        #[namespace = "mlx_cxx"]
        fn key(seed: u64) -> UniquePtr<array>;

        #[namespace = "mlx::core::random"]
        fn seed(seed: u64);

        #[namespace = "mlx_cxx"]
        #[rust_name = "bits_with_width"]
        fn bits(
            shape: &CxxVector<i32>,
            width: i32,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bits(
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn split(
            key: &array,
            s: StreamOrDevice,
        ) -> [UniquePtr<array>; 2];

        #[namespace = "mlx_cxx"]
        #[rust_name = "split_n"]
        fn split(
            key: &array,
            n: i32,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform(
            low: &array,
            high: &array,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_bool(
            low: bool,
            high: bool,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_uint8(
            low: u8,
            high: u8,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_uint16(
            low: u16,
            high: u16,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_uint32(
            low: u32,
            high: u32,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_uint64(
            low: u64,
            high: u64,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_int8(
            low: i8,
            high: i8,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_int16(
            low: i16,
            high: i16,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_int32(
            low: i32,
            high: i32,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_int64(
            low: i64,
            high: i64,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_float16(
            low: float16_t,
            high: float16_t,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_bfloat16(
            low: bfloat16_t,
            high: bfloat16_t,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_float32(
            low: f32,
            high: f32,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn uniform_complex64(
            low: complex64_t,
            high: complex64_t,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "normal_with_dtype"]
        fn normal(
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn normal(
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint(
            low: &array,
            high: &array,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_bool(
            low: bool,
            high: bool,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_uint8(
            low: u8,
            high: u8,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_uint16(
            low: u16,
            high: u16,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_uint32(
            low: u32,
            high: u32,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_uint64(
            low: u64,
            high: u64,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_int8(
            low: i8,
            high: i8,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_int16(
            low: i16,
            high: i16,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_int32(
            low: i32,
            high: i32,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_int64(
            low: i64,
            high: i64,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_float16(
            low: float16_t,
            high: float16_t,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_bfloat16(
            low: bfloat16_t,
            high: bfloat16_t,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_float32(
            low: f32,
            high: f32,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn randint_complex64(
            low: complex64_t,
            high: complex64_t,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape"]
        fn bernoulli(
            p: &array,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli"]
        fn bernoulli(
            p: &array,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_bool(
            p: bool,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;
        
        #[namespace = "mlx_cxx"]
        fn bernoulli_uint8(
            p: u8,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_uint16(
            p: u16,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_uint32(
            p: u32,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_uint64(
            p: u64,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_int8(
            p: i8,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_int16(
            p: i16,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_int32(
            p: i32,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_int64(
            p: i64,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_float16(
            p: float16_t,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_bfloat16(
            p: bfloat16_t,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_float32(
            p: f32,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn bernoulli_complex64(
            p: complex64_t,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_bool"]
        fn bernoulli_bool(
            p: bool,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_uint8"]
        fn bernoulli_uint8(
            p: u8,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_uint16"]
        fn bernoulli_uint16(
            p: u16,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_uint32"]
        fn bernoulli_uint32(
            p: u32,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_uint64"]
        fn bernoulli_uint64(
            p: u64,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_int8"]
        fn bernoulli_int8(
            p: i8,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_int16"]
        fn bernoulli_int16(
            p: i16,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_int32"]
        fn bernoulli_int32(
            p: i32,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_int64"]
        fn bernoulli_int64(
            p: i64,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_float16"]
        fn bernoulli_float16(
            p: float16_t,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_bfloat16"]
        fn bernoulli_bfloat16(
            p: bfloat16_t,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_float32"]
        fn bernoulli_float32(
            p: f32,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "bernoulli_with_shape_complex64"]
        fn bernoulli_complex64(
            p: complex64_t,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        // TODO: ignore bernoulli with 0.5 probability for now

        #[namespace = "mlx_cxx"]
        #[rust_name = "truncate_normal_with_shape"]
        fn truncated_normal(
            lower: &array,
            upper: &array,
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn truncated_normal(
            lower: &array,
            upper: &array,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn gumbel(
            shape: &CxxVector<i32>,
            dtype: Dtype,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "categorical_with_shape"]
        fn categorical(
            logits: &array,
            axis: i32,
            shape: &CxxVector<i32>,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "categorical_with_num_samples"]
        fn categorical(
            logits: &array,
            axis: i32,
            num_samples: i32,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn categorical(
            logits: &array,
            axis: i32,
            key: &OptionalArray,
            s: StreamOrDevice,
        ) -> UniquePtr<array>;
    }
}