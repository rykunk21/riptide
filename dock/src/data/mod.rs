///
/// This file defines common data management functions for csv files and polars
///
use polars::prelude::*;

pub mod pipeline;

pub enum DataType {
    Ordinal,
    Nominal,
    Interval,
    Ratio,
}

pub fn get_data_frame(file_path: &str) -> PolarsResult<DataFrame> {
    LazyCsvReader::new(file_path)
        .with_n_rows(Some(1000))
        .finish()?
        .collect()
}

trait ExtDataFrame {
    fn dist(&self) -> ();
}

impl ExtDataFrame for DataFrame {
    fn dist(&self) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_open_csv() {
        let df = get_data_frame("examples/data.csv").expect("Failed to load CSV file");
        println!("{:?}", df.schema());
    }
    #[test]
    pub fn test_read_csv() {
        let ex = df![
            "age" => &[23i64, 31, 27, 45],
            "gender" => &["M", "F", "F", "M"],
            "income" => &[54000i64, 61000, 58000, 72000],
            "region" => &[3, 2, 14, 5]
        ]
        .expect("Cannot create test df");

        let df = get_data_frame("examples/data.csv").expect("Failed to load CSV file");
        assert_eq!(df, ex);
    }
}
