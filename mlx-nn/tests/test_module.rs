use mlx_macros::ModuleParameters;
use mlx_nn::Linear;
use mlx_rs::module::Module;
use mlx_rs::{error::Exception, Array};

#[derive(Debug, Clone, ModuleParameters)]
struct M {
    #[param]
    linear: Linear,
}

impl M {
    pub fn new() -> Self {
        Self {
            linear: Linear::new(5, 5).unwrap(),
        }
    }
}

impl<'a> Module<&'a Array> for M {
    type Error = Exception;
    type Output = Array;

    fn forward(&mut self, x: &Array) -> Result<Array, Self::Error> {
        self.linear.forward(x)
    }

    fn training_mode(&mut self, _mode: bool) {}
}

#[test]
fn test_nested_module() {
    let mut m = M::new();
    let x = mlx_rs::random::uniform::<_, f32>(1.0, 2.0, &[1, 5], None).unwrap();
    let y = m.forward(&x).unwrap();
    assert_ne!(y.sum(None, None).unwrap(), mlx_rs::array!(0.0));
}
