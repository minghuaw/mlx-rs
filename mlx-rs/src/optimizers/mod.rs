//! Trait and implementations for optimizers.

#![deny(missing_docs)]

use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    rc::Rc,
};

use crate::{
    array,
    module::{FlattenedModuleParam, ModuleParameters},
    Array,
};

mod adadelta;
mod adafactor;
mod adagrad;
mod adam;
mod adamax;
mod adamw;
mod lion;
mod rmsprop;
mod sgd;

pub use adadelta::*;
pub use adafactor::*;
pub use adagrad::*;
pub use adam::*;
pub use adamax::*;
pub use adamw::*;
pub use lion::*;
pub use rmsprop::*;
pub use sgd::*;

type OptimizerState<T = Array> = HashMap<Rc<str>, T>;

/// Trait for optimizers.
pub trait Optimizer {
    /// Update a single parameter with the given gradient.
    ///
    /// The implementation should look up the state for the parameter using the key and update the
    /// state and the parameter accordingly. The key is provided instead of the state because it
    /// would otherwise create a mutable borrow conflict with the rest of the optimizer fields.
    fn apply_single(
        &mut self,
        key: &Rc<str>,
        gradient: &Array,
        parameter: &mut Array,
    ) -> crate::error::Result<()>;

    /// Apply the gradients to the parameters of the model and update the model with the new
    /// parameters.
    fn apply<M>(
        &mut self,
        model: &mut M,
        gradients: impl Borrow<FlattenedModuleParam>,
    ) -> crate::error::Result<()>
    where
        M: ModuleParameters,
    {
        let mut parameters = model.parameters_mut().flatten();

        for (key, gradient) in gradients.borrow().iter() {
            if let Some(parameter) = parameters.get_mut(key) {
                self.apply_single(key, gradient, parameter)?;
            }
        }

        Ok(())
    }
}

/// Type alias for clipped gradients that is returned by `clip_grad_norm`.
pub type MaybeClippedGrads<'a> = HashMap<Rc<str>, Cow<'a, Array>>;

/// Clips the global norm of the gradients
///
/// This function ensures that the global norm of the gradients does not exceed
/// `max_norm`. It scales down the gradients proportionally if their norm is
/// greater than `max_norm`.
pub fn clip_grad_norm(
    gradients: &FlattenedModuleParam,
    max_norm: f32,
) -> crate::error::Result<(MaybeClippedGrads, f32)> {
    let total_norm: f32 = gradients
        .values()
        .try_fold(array!(0.0), |acc, grad| {
            acc.add(&grad.square()?.sum(None, None)?)
        })?
        .sqrt()?
        .item();
    let normalizer = array!(max_norm / (total_norm + 1e-6));

    let clipped_gradients: HashMap<_, _> = gradients
        .iter()
        .map(|(key, grad)| {
            let clipped_grad = if total_norm < max_norm {
                Cow::Borrowed(grad)
            } else {
                Cow::Owned(grad * &normalizer)
            };
            (key.clone(), clipped_grad)
        })
        .collect();
    Ok((clipped_gradients, total_norm))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{array, module::FlattenedModuleParam, Array};

    use super::clip_grad_norm;

    #[test]
    fn test_clip_grad_norm() {
        // Test with small gradients that do not require clipping
        let mut small_grads: FlattenedModuleParam = HashMap::new();
        small_grads.insert("first.a".into(), array!([0.1, 0.2]));
        small_grads.insert("first.b".into(), array!(0.1));
        small_grads.insert("second".into(), array!(0.3));

        let max_norm = 10.0;

        let (clipped_grads, _) = clip_grad_norm(&small_grads, max_norm).unwrap();
        for (key, value) in small_grads.iter() {
            assert_eq!(&*clipped_grads[key], value);
        }

        // Test with large gradients that require clipping
        let mut large_grads: FlattenedModuleParam = HashMap::new();
        large_grads.insert("first.a".into(), array!([10.0, 20.0]));
        large_grads.insert("first.b".into(), array!(10.0));
        large_grads.insert("second".into(), array!(30.0));

        let max_norm = 1.0;

        let (clipped_grads, total_norm) = clip_grad_norm(&large_grads, max_norm).unwrap();
        let clipped_values: Vec<_> = clipped_grads.values().map(|v| v.as_ref()).collect();
        let norm_of_clipped = clipped_values
            .into_iter()
            .map(|g| g.square().unwrap().sum(None, None).unwrap())
            .sum::<Array>()
            .sqrt()
            .unwrap();

        float_eq::assert_float_eq!(norm_of_clipped.item::<f32>(), max_norm, abs <= 1e-6);

        // Ensures that the scaling was done correctly
        let scale = max_norm / total_norm;
        let expected_grads: FlattenedModuleParam = large_grads
            .iter()
            .map(|(key, value)| (key.clone(), value * scale))
            .collect();
        for (key, value) in expected_grads.iter() {
            assert_eq!(&*clipped_grads[key], value);
        }
    }
}
