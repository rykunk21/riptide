use burn::{
    nn::{Linear, LinearConfig, Relu}, prelude::*
};


/// Perceptron and basic linear elements
#[derive(Module, Debug)]
pub struct Perceptron<B: Backend> {
    linear: Linear<B>,
    activation: Relu,
}

impl<B: Backend> Perceptron<B> {
    pub fn forward(&self, features: Tensor<B, 1>) -> Tensor<B, 1> {
        // reshape?

        // begin forward prop
        let x = self.linear.forward(features);
        self.activation.forward(x)
    }

}


#[derive(Config, Debug)]
pub struct PerceptronConfig {
    input_size: usize,
    output_size: usize,
}

impl PerceptronConfig {
    #[allow(dead_code)]
    pub fn init<B: Backend>(&self, device: &B::Device) -> Perceptron<B> {
        Perceptron {
            linear: LinearConfig::new(self.input_size, self.output_size).init(device),
            activation: Relu::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burn::backend::Wgpu;
    use polars::df;

    #[test]
    fn test_perceptron() {
        type Backend = Wgpu<f32, i32>;

        let device = Default::default();
        let _ = PerceptronConfig::new(1, 1).init::<Backend>(&device);
    }

    #[test]
    fn test_train_perceptron() {
        type Backend = Wgpu<f32, i32>;
        let device = Default::default();
        let model = PerceptronConfig::new(1, 1).init::<Backend>(&device);

        // this plot represents a linearly separable square
        let df = df![
            "x1" => [0.5, 1.1, -0.3, -1.0],
            "x2" => [1.2, 0.8, 0.7, -0.5],
            "y" => [1, -1, 1, -1]
        ];

        



    }
}
