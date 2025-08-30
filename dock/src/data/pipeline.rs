use polars::prelude::*;
pub enum Stage<'a> {
    Imputer(&'a str),
    Scaler(&'a str),
    Encoder,
    PCA(i32),
}

pub struct Pipeline {
    stages: Vec<Stage<'static>>,
}

impl Pipeline {
    pub fn new(stages: Vec<Stage<'static>>) -> Self {
        Pipeline { stages }
    }

    pub fn fit(&self, df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn transform(&self, df: &DataFrame) -> Result<DataFrame, Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn fit_transform(&self, df: &DataFrame) -> Result<DataFrame, Box<dyn std::error::Error>> {
        self.fit(df)?;
        self.transform(df)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_usage() {
        // Construct df
        let df = df![
            "x1" => [0.5, 1.1, -0.3, -1.0],
            "x2" => [1.2, 0.8, 0.7, -0.5],
            "y" => [1, -1, 1, -1]
        ]
        .expect("Could not construct dataframe");

        // Create transformers
        let pipeline = Pipeline::new(vec![
            Stage::Imputer("mean"),
            Stage::Scaler("minmax"),
            Stage::PCA(2),
        ]);

        let transformed_df = pipeline
            .fit_transform(&df)
            .expect("Could not transform dataframe");

        println!("{}", transformed_df);
    }
}
