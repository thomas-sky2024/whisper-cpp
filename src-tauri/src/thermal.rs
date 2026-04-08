

/// Performance mode for the pipeline.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PerformanceMode {
    Balanced,
    MaxSpeed,
}
