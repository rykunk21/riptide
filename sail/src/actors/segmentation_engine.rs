use burn::prelude::*;
use polars::prelude::*;
/// This file describes the segmentation engine
use std::error::Error;
use thiserror::Error;

/// Actor
pub enum SegmentationEngine {
    Idle,
    Running,
    Completed(util::IFC),
    Failed(SegmentationError),
}

/// Message
#[derive(Debug, Clone)]
pub enum SegmentationEngineMsg {
    Start {
        file_path: String,
        params: SegmentationParams,
    },
    Export {
        output_path: String,
    },
}

/// Actor Parameters
#[derive(Debug, Default, Clone)]
pub struct SegmentationParams {
    voxel_size: f32,
}

/// Errors
#[derive(Debug, Error)]
pub enum SegmentationError {
    #[error("Load error: {0}")]
    LoadError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Incomplete operation: {0}")]
    Incomplete(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    #[error("Cannot generate dataframe from supplied data: {0}")]
    DataFrameError(String),
}

/// Implmentation
impl SegmentationEngine {
    /// Initialize a new SegmentationEngine
    pub fn new() -> Self {
        Self::Idle
    }

    /// Main entry point to handle messages to the enum
    pub fn send(&mut self, msg: SegmentationEngineMsg) -> Result<(), SegmentationError> {
        match msg {
            SegmentationEngineMsg::Start { file_path, params } => {
                match self.segment(&file_path, &params) {
                    Ok(ifc) => {
                        *self = SegmentationEngine::Completed(ifc);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
            SegmentationEngineMsg::Export { output_path } => {
                self.export(Some(&output_path))?;
                Ok(())
            }
        }
    }
    /// Main segmenting logic, translates point cloud data to semantically rich ifc
    /// Semantic labeler should be separate?
    /// How do I prevent this function from becoming huge?
    fn segment(
        &self,
        file: &str,
        params: &SegmentationParams,
    ) -> Result<self::util::IFC, SegmentationError> {
        util::load(file)?;

        todo!();
    }
    /// Saves the .ifc file to the exported path

    fn export(&self, path: Option<&str>) -> Result<util::IFC, SegmentationError> {
        match self {
            SegmentationEngine::Completed(output) => util::save(path, output),
            _ => Err(SegmentationError::Incomplete(
                "The segmentation has not started / is still incomplete".into(),
            )),
        }
    }
}

/// Segmentation utility functions
/// IO operations
/// Math
mod util {

    use super::*;
    use e57::{CartesianCoordinate, E57Reader, RecordValue};
    use polars::prelude::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use polars::prelude::DataFrame;
    use serde_json::Value as Json;

    pub type E57 = DataFrame;
    pub type IFC = Json;

    pub fn load(file: &str) -> Result<DataFrame, SegmentationError> {
        // open the file and read into buf reader. E57 files usually contain a lot of data so
        // buffering is necessary to prevent failures
        let f = File::open(file).map_err(|e| {
            SegmentationError::FileNotFound(format!("Could not open {}: {}", file, e))
        })?;

        let reader = BufReader::new(f);

        // Create an E57 reader instance
        let mut e57_reader = E57Reader::new(reader).map_err(|e| {
            SegmentationError::InvalidFormat(format!("Invalid E57 format: {:?}", e))
        })?;

        let pointclouds = e57_reader.pointclouds();
        println!("Found {} point clouds in the file.", pointclouds.len());
        for (i, pointcloud) in pointclouds.iter().enumerate() {
            // Prepare vectors for each column
            let mut xs = Vec::new();
            let mut ys = Vec::new();
            let mut zs = Vec::new();
            let mut rs = Vec::new();
            let mut gs = Vec::new();
            let mut bs = Vec::new();

            for point_result in e57_reader.pointcloud_raw(pointcloud).unwrap() {
                let point = point_result.unwrap(); // unwrap Result
                                                   // point[0..2] are Single(f32/f64), point[3..5] are Integer(i32)
                if let [RecordValue::Single(x), RecordValue::Single(y), RecordValue::Single(z), RecordValue::Integer(r), RecordValue::Integer(g), RecordValue::Integer(b)] =
                    &point[..]
                {
                    xs.push(*x as f64); // Polars uses f64 for Float64Chunked
                    ys.push(*y as f64);
                    zs.push(*z as f64);
                    rs.push(*r);
                    gs.push(*g);
                    bs.push(*b);
                }
            }

            let df = DataFrame::new(vec![
                Series::new("x", xs),
                Series::new("y", ys),
                Series::new("z", zs),
                Series::new("r", rs),
                Series::new("g", gs),
                Series::new("b", bs),
            ])
            .map_err(|e| SegmentationError::DataFrameError(e.to_string()))?;
            println!("{:?}", df);

            return Ok(df);
        }

        // Temp
        Err(SegmentationError::LoadError("Exit".into()))
    }

    /// Saves the ifc file to the location
    pub fn save(file: Option<&str>, ifc: &IFC) -> Result<IFC, SegmentationError> {
        todo!();
    }

    /// Create a dummy ifc output for testing
    pub fn dummy_ifc() -> IFC {
        todo!();
    }

    /// has labels
    pub fn has_labels(ifc: &IFC, labels: &[&str]) -> bool {
        todo!();
    }

    /// This function allows the raw segmentation attempt to be semantically enriched using
    /// various methods TBD
    pub fn enrich(e57: E57, params: SegmentationParams) -> Result<IFC, SegmentationError> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        // Client instantiates the SegmentationEngine in its Idle state
        let mut engine = SegmentationEngine::new();

        // Client prepares parameters for segmentation
        let params = SegmentationParams::default();

        // Client sends a "Start" message with a valid input file
        let start_result = engine.send(SegmentationEngineMsg::Start {
            file_path: "test/segmentation/test.e57".into(),
            params,
        });

        assert!(start_result.is_ok(), "Segmentation should start succestest");

        // Simulate processing (in real use this might be async / background task)
        match &engine {
            SegmentationEngine::Running => {
                // Pretend segmentation finishes immediately for the test
                // Normally you'd have engine internally transition to Completed
            }
            _ => {}
        }

        // Manually move to completed state for this test (stubbed pipeline)
        engine = SegmentationEngine::Completed(util::dummy_ifc());

        // Client now requests an export to IFC
        let export_result = engine.send(SegmentationEngineMsg::Export {
            output_path: "output.ifc".into(),
        });

        assert!(
            export_result.is_ok(),
            "Export should succeed once Completed"
        );
    }

    /// FR-001, FR-002: Engine should start only if Idle and input file valid
    #[test]
    fn test_start_from_idle_with_valid_file() {
        let mut seg = SegmentationEngine::new();
        let params = SegmentationParams::default();
        let result = seg.send(SegmentationEngineMsg::Start {
            file_path: "test_data/valid.e57".into(),
            params,
        });
        assert!(result.is_ok());
        match seg {
            SegmentationEngine::Running | SegmentationEngine::Completed(_) => {}
            _ => panic!("Engine did not transition correctly"),
        }
    }

    /// FR-002: Engine rejects invalid file path
    #[test]
    fn test_start_with_invalid_file() {
        let mut seg = SegmentationEngine::new();
        let params = SegmentationParams::default();
        let result = seg.send(SegmentationEngineMsg::Start {
            file_path: "missing_file.e57".into(),
            params,
        });
        assert!(result.is_err());
        match seg {
            SegmentationEngine::Failed(_) => {}
            _ => panic!("Engine should have failed"),
        }
    }

    /// Contract: cannot start again unless Idle
    #[test]
    fn test_start_when_not_idle() {
        let mut engine = SegmentationEngine::new();
        let params = SegmentationParams::default();
        // First start (ok)
        engine
            .send(SegmentationEngineMsg::Start {
                file_path: "test_data/valid.e57".into(),
                params: params.clone(),
            })
            .unwrap();
        // Second start should fail unless reset
        let result = engine.send(SegmentationEngineMsg::Start {
            file_path: "test_data/valid.e57".into(),
            params: params.clone(),
        });
        assert!(result.is_err());
    }

    /// FR-004, FR-006: Export IFC succeeds only after Completed
    #[test]
    fn test_export_after_completion() {
        let mut seg = SegmentationEngine::Completed(self::util::dummy_ifc());
        let result = seg.send(SegmentationEngineMsg::Export {
            output_path: "out.ifc".into(),
        });
        assert!(result.is_ok());
    }

    /// Contract: Export should fail if not Completed
    #[test]
    fn test_export_when_incomplete() {
        let mut seg = SegmentationEngine::Running;
        let result = seg.send(SegmentationEngineMsg::Export {
            output_path: "out.ifc".into(),
        });
        assert!(result.is_err());
    }

    /// Utility: Parsing valid file produces E57 DataFrame
    #[test]
    fn test_parse_valid_file() {
        let result = util::load("test/segmentation/test.e57");

        if !result.is_ok() {
            panic!("Failed to parse valid file: {:?}", result.err());
        }
        assert!(result.is_ok());
    }

    /// Utility: Parsing invalid file produces error
    #[test]
    fn test_parse_invalid_file() {
        let result = util::load("test_data/corrupt.e57");
        assert!(result.is_err());
    }

    /// Utility: Saving IFC returns Ok when file path provided
    #[test]
    fn test_save_ifc_to_path() {
        let ifc = util::dummy_ifc();
        let result = util::save(Some("out.ifc"), &ifc);
        assert!(result.is_ok());
    }

    /// Utility: Saving IFC returns IFC when no path provided
    #[test]
    fn test_save_ifc_in_memory() {
        let ifc = util::dummy_ifc();
        let result = util::save(None, &ifc).unwrap();
        // should return same IFC object when no path is given
        assert_eq!(result, ifc);
    }

    /// FR-003: Semantic segmentation produces labeled IFC
    #[test]
    fn test_segmentation_labels_applied() {
        let params = SegmentationParams::default();
        let e57 = util::load("test_data/valid.e57").unwrap();
        let ifc = util::enrich(e57, params).unwrap();
        assert!(util::has_labels(&ifc, &["Wall", "Floor", "Slab"]));
    }

    /// FR-005: Parameters influence output
    #[test]
    fn test_segmentation_params_affect_output() {
        let mut params_a = SegmentationParams::default();
        let mut params_b = SegmentationParams::default();
        params_a.voxel_size = 0.1;
        params_b.voxel_size = 0.5;

        let e57 = util::load("test_data/valid.e57").unwrap();
        let ifc_a = util::enrich(e57.clone(), params_a).unwrap();
        let ifc_b = util::enrich(e57, params_b).unwrap();

        assert_ne!(
            ifc_a, ifc_b,
            "Different params should yield different results"
        );
    }
}
