pub mod colors;

#[cfg(test)]
mod test {
    use burn::{backend::Wgpu, data::dataset::Dataset, tensor::Tensor};
    use burn_dataset::DataframeDataset;
    use polars::prelude::*;
    use serde::Deserialize;

    #[test]
    fn test_tensors() {
        type Backend = Wgpu<f32, i32>;
        let device = burn::backend::wgpu::WgpuDevice::default();

        let floats = [1.0, 2.0, 3.0];
        let tensor_1 = Tensor::<Backend, 1>::from_floats(floats, &device);

        println!("Test tensor loaded on WGPU: {}", tensor_1)
    }

    #[test]
    fn text_convert_df_to_dataset() {
        #[derive(Debug, Clone, Deserialize, PartialEq)]
        struct Item {
            feature: String,
            label: String,
        }

        let testdata = df![
            "feature" => ["1","2","3"],
            "random" => ["3","10", "25"],
            "label"=> ["4","5","6"],
            "x" => [1,2,3],
            "y" => [4,5,6]
        ]
        .expect("Could not construct dataframe");

        let df = DataframeDataset::<Item>::new(testdata).expect("Cannot create DF dataset");

        let r1 = df.get(0).expect("cannot deserialize Row 1");

        assert_eq!(
            r1,
            Item {
                feature: "1".to_string(),
                label: "4".to_string()
            }
        );
    }
}
