use cxx::{CxxVector, UniquePtr};

use crate::array::ffi::array;

use self::ffi::CxxUnaryFn;

pub trait Function<Args> {
    type Output;

    fn execute(&self, args: Args) -> Self::Output;
}

impl<Args, F, R> Function<Args> for F
where
    F: Fn(Args) -> R,
{
    type Output = R;

    fn execute(&self, args: Args) -> Self::Output {
        self(args)
    }
}

pub trait CompatFn {
    type CxxFn;
}

#[repr(transparent)]
pub struct UnaryFn(pub Box<dyn for<'a> Function<&'a array, Output = UniquePtr<array>> + 'static>);

impl CompatFn for UnaryFn {
    type CxxFn = CxxUnaryFn;
}

impl<F> From<F> for UnaryFn
where
    F: for<'a> Function<&'a array, Output = UniquePtr<array>> + 'static,
{
    fn from(f: F) -> Self {
        Self(Box::new(f))
    }
}

pub fn execute_unary_fn(f: &UnaryFn, args: &array) -> UniquePtr<array> {
    f.0.execute(args)
}

#[repr(transparent)]
pub struct MultiaryFn(
    pub  Box<
        dyn for<'a> Function<&'a CxxVector<array>, Output = UniquePtr<CxxVector<array>>> + 'static,
    >,
);

impl CompatFn for MultiaryFn {
    type CxxFn = ffi::CxxMultiaryFn;
}

impl<F> From<F> for MultiaryFn
where
    F: for<'a> Function<&'a CxxVector<array>, Output = UniquePtr<CxxVector<array>>> + 'static,
{
    fn from(f: F) -> Self {
        Self(Box::new(f))
    }
}

pub fn execute_multiary_fn(f: &MultiaryFn, args: &CxxVector<array>) -> UniquePtr<CxxVector<array>> {
    f.0.execute(args)
}

#[repr(transparent)]
pub struct MultiInputSingleOutputFn(
    pub Box<dyn for<'a> Function<&'a CxxVector<array>, Output = UniquePtr<array>> + 'static>,
);

impl CompatFn for MultiInputSingleOutputFn {
    type CxxFn = ffi::CxxMultiInputSingleOutputFn;
}

impl<F> From<F> for MultiInputSingleOutputFn
where
    F: for<'a> Function<&'a CxxVector<array>, Output = UniquePtr<array>> + 'static,
{
    fn from(f: F) -> Self {
        Self(Box::new(f))
    }
}

fn execute_multi_input_single_output_fn(
    f: &MultiInputSingleOutputFn,
    args: &CxxVector<array>,
) -> UniquePtr<array> {
    f.0.execute(args)
}

#[repr(transparent)]
pub struct PairInputSingleOutputFn(
    pub Box<dyn for<'a> Function<[&'a array; 2], Output = UniquePtr<array>> + 'static>,
);

impl CompatFn for PairInputSingleOutputFn {
    type CxxFn = ffi::CxxPairInputSingleOutputFn;
}

impl<F> From<F> for PairInputSingleOutputFn
where
    F: for<'a> Function<[&'a array; 2], Output = UniquePtr<array>> + 'static,
{
    fn from(f: F) -> Self {
        Self(Box::new(f))
    }
}

// This runs into problem with the c++ binding if we use [&array; 2] as the input type
fn execute_pair_input_single_output_fn(
    f: &PairInputSingleOutputFn,
    first: &array,
    second: &array,
) -> UniquePtr<array> {
    f.0.execute([first, second])
}

#[repr(transparent)]
pub struct VjpFn(
    pub  Box<
        dyn for<'a> Function<[&'a CxxVector<array>; 3], Output = UniquePtr<CxxVector<array>>>
            + 'static,
    >,
);

impl CompatFn for VjpFn {
    type CxxFn = ffi::CxxMultiaryFn;
}

impl<F> From<F> for VjpFn
where
    F: for<'a> Function<[&'a CxxVector<array>; 3], Output = UniquePtr<CxxVector<array>>> + 'static,
{
    fn from(f: F) -> Self {
        Self(Box::new(f))
    }
}

fn execute_vjp_fn(
    f: &VjpFn,
    arg1: &CxxVector<array>,
    arg2: &CxxVector<array>,
    arg3: &CxxVector<array>,
) -> UniquePtr<CxxVector<array>> {
    f.0.execute([arg1, arg2, arg3])
}

// TODO: change visibility and then re-export
#[cxx::bridge]
pub(crate) mod ffi {
    extern "C++" {
        include!("mlx/array.h");

        #[namespace = "mlx::core"]
        type array = crate::array::ffi::array;
    }

    extern "Rust" {
        #[namespace = "mlx_cxx"]
        type UnaryFn;

        #[namespace = "mlx_cxx"]
        type MultiaryFn;

        #[namespace = "mlx_cxx"]
        type MultiInputSingleOutputFn;

        #[namespace = "mlx_cxx"]
        type PairInputSingleOutputFn;

        #[namespace = "mlx_cxx"]
        type VjpFn;

        #[namespace = "mlx_cxx"]
        fn execute_unary_fn(f: &UnaryFn, x: &array) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn execute_multiary_fn(
            f: &MultiaryFn,
            xs: &CxxVector<array>,
        ) -> UniquePtr<CxxVector<array>>;

        #[namespace = "mlx_cxx"]
        fn execute_multi_input_single_output_fn(
            f: &MultiInputSingleOutputFn,
            xs: &CxxVector<array>,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn execute_pair_input_single_output_fn(
            f: &PairInputSingleOutputFn,
            first: &array,
            second: &array,
        ) -> UniquePtr<array>;

        #[namespace = "mlx_cxx"]
        fn execute_vjp_fn(
            f: &VjpFn,
            arg1: &CxxVector<array>,
            arg2: &CxxVector<array>,
            arg3: &CxxVector<array>,
        ) -> UniquePtr<CxxVector<array>>;
    }

    unsafe extern "C++" {
        /* -------------------------------------------------------------------------- */
        /*                          bindings for transforms.h                         */
        /* -------------------------------------------------------------------------- */

        // include!("mlx-cxx/transforms.hpp");
        include!("mlx-cxx/compat.hpp");

        #[namespace = "mlx_cxx"]
        type CxxUnaryFn = crate::transforms::ffi::CxxUnaryFn;

        #[namespace = "mlx_cxx"]
        type CxxMultiaryFn = crate::transforms::ffi::CxxMultiaryFn;

        #[namespace = "mlx_cxx"]
        type CxxMultiInputSingleOutputFn = crate::transforms::ffi::CxxMultiInputSingleOutputFn;

        #[namespace = "mlx_cxx"]
        type CxxPairInputSingleOutputFn = crate::transforms::ffi::CxxPairInputSingleOutputFn;

        #[namespace = "mlx_cxx"]
        type CxxSingleInputPairOutputFn = crate::transforms::ffi::CxxSingleInputPairOutputFn;

        #[namespace = "mlx::core"]
        #[cxx_name = "ValueAndGradFn"]
        type CxxValueAndGradFn = crate::transforms::ffi::CxxValueAndGradFn;

        #[namespace = "mlx::core"]
        #[cxx_name = "SimpleValueAndGradFn"]
        type CxxSimpleValueAndGradFn = crate::transforms::ffi::CxxSimpleValueAndGradFn;

        #[namespace = "mlx_cxx"]
        #[rust_name = "vjp_unary_fn"]
        unsafe fn vjp(
            f: *const UnaryFn,
            primal: &array,
            cotangent: &array,
        ) -> Result<[UniquePtr<array>; 2]>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "vjp_multiary_fn"]
        unsafe fn vjp(
            f: *const MultiaryFn,
            primal: &CxxVector<array>,
            cotangent: &CxxVector<array>,
        ) -> Result<[UniquePtr<CxxVector<array>>; 2]>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "jvp_unary_fn"]
        unsafe fn jvp(
            f: *const UnaryFn,
            primal: &array,
            tangent: &array,
        ) -> Result<[UniquePtr<array>; 2]>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "jvp_multiary_fn"]
        unsafe fn jvp(
            f: *const MultiaryFn,
            primal: &CxxVector<array>,
            tangent: &CxxVector<array>,
        ) -> Result<[UniquePtr<CxxVector<array>>; 2]>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "value_and_grad_multiary_fn_argnums"]
        unsafe fn value_and_grad(
            f: *const MultiaryFn,
            argnums: &CxxVector<i32>,
        ) -> Result<UniquePtr<CxxValueAndGradFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "value_and_grad_multiary_fn_argnum"]
        unsafe fn value_and_grad(
            f: *const MultiaryFn,
            argnum: i32,
        ) -> Result<UniquePtr<CxxValueAndGradFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "value_and_grad_unary_fn"]
        unsafe fn value_and_grad(
            f: *const UnaryFn,
        ) -> Result<UniquePtr<CxxSingleInputPairOutputFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "value_and_grad_multi_input_single_output_fn"]
        unsafe fn value_and_grad(
            f: *const MultiInputSingleOutputFn,
            argnums: &CxxVector<i32>,
        ) -> Result<UniquePtr<CxxSimpleValueAndGradFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "grad_multi_input_single_output_fn_argnums"]
        unsafe fn grad(
            f: *const MultiInputSingleOutputFn,
            argnums: &CxxVector<i32>,
        ) -> Result<UniquePtr<CxxMultiaryFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "grad_multi_input_single_output_fn_argnum"]
        unsafe fn grad(
            f: *const MultiInputSingleOutputFn,
            argnum: i32,
        ) -> Result<UniquePtr<CxxMultiaryFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "grad_unary_fn"]
        unsafe fn grad(f: *const UnaryFn) -> Result<UniquePtr<CxxUnaryFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "vmap_unary_fn"]
        unsafe fn vmap(
            f: *const UnaryFn,
            in_axis: i32,
            out_axis: i32,
        ) -> Result<UniquePtr<CxxUnaryFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "vmap_pair_input_single_output_fn"]
        unsafe fn vmap(
            f: *const PairInputSingleOutputFn,
            in_axis_a: i32,
            in_axis_b: i32,
            out_axis: i32,
        ) -> Result<UniquePtr<CxxPairInputSingleOutputFn>>;

        #[namespace = "mlx_cxx"]
        #[rust_name = "vmap_multiary_fn"]
        unsafe fn vmap(
            f: *const MultiaryFn,
            in_axes: &CxxVector<i32>,
            out_axes: &CxxVector<i32>,
        ) -> Result<UniquePtr<CxxMultiaryFn>>;

        #[namespace = "mlx_cxx"]
        unsafe fn custom_vjp(
            fun: *const MultiaryFn,
            fun_vjp: *const VjpFn,
        ) -> Result<UniquePtr<CxxMultiaryFn>>;

        #[namespace = "mlx_cxx"]
        unsafe fn checkpoint(fun: *const MultiaryFn) -> Result<UniquePtr<CxxMultiaryFn>>;

        /* -------------------------------------------------------------------------- */
        /*                           bindings for compile.h                           */
        /* -------------------------------------------------------------------------- */

        // include!("mlx-cxx/compile.hpp");

        #[namespace = "mlx_cxx"]
        unsafe fn compile(fun: *const MultiaryFn, shapeless: bool) -> Result<UniquePtr<CxxMultiaryFn>>;
    }
}
