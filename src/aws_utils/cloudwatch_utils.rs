/// Constants for dimension names and values
pub const SERVICE_DIMENSION: &str = "Service";
pub const PRIORITY_EXECUTOR: &str = "PriorityExecutor";
pub const V2_EXECUTOR: &str = "V2Executor";

/// Constants for metric names
pub const TX_SUCCEEDED_METRIC: &str = "TransactionSucceeded";
pub const TX_REVERTED_METRIC: &str = "TransactionReverted";
pub const TX_SUBMITTED_METRIC: &str = "TransactionSubmitted";
pub const TX_STATUS_UNKNOWN_METRIC: &str = "TransactionStatusUnknown";
pub const LATEST_BLOCK: &str = "LatestBlock";

pub enum DimensionName {
    Service,
}
impl AsRef<str> for DimensionName {
    fn as_ref(&self) -> &str {
        match self {
            DimensionName::Service => SERVICE_DIMENSION,
        }
    }
}

impl From<DimensionName> for String {
    fn from(dimension: DimensionName) -> Self {
        match dimension {
            DimensionName::Service => SERVICE_DIMENSION.to_string(),
        }
    }
}

pub enum DimensionValue {
    PriorityExecutor,
    V2Executor,
}
impl From<DimensionValue> for String {
    fn from(value: DimensionValue) -> Self {
        match value {
            DimensionValue::PriorityExecutor => PRIORITY_EXECUTOR.to_string(),
            DimensionValue::V2Executor => V2_EXECUTOR.to_string(),
        }
    }
}

impl AsRef<str> for DimensionValue {
    fn as_ref(&self) -> &str {
        match self {
            DimensionValue::PriorityExecutor => PRIORITY_EXECUTOR,
            DimensionValue::V2Executor => V2_EXECUTOR,
        }
    }
}

pub enum CwMetrics {
    TxSucceeded,
    TxReverted,
    TxSubmitted,
    TxStatusUnknown,
    LatestBlock,

    /// Balance for individual address
    Balance(String),
}
impl From<CwMetrics> for String {
    fn from(metric: CwMetrics) -> Self {
        match metric {
            CwMetrics::TxSucceeded => TX_SUCCEEDED_METRIC.to_string(),
            CwMetrics::TxReverted => TX_REVERTED_METRIC.to_string(),
            CwMetrics::TxSubmitted => TX_SUBMITTED_METRIC.to_string(),
            CwMetrics::TxStatusUnknown => TX_STATUS_UNKNOWN_METRIC.to_string(),
            CwMetrics::Balance(val) => format!("Bal-{}", val),
            CwMetrics::LatestBlock => LATEST_BLOCK.to_string(),
        }
    }
}

use aws_sdk_cloudwatch::types::Dimension;

pub const ARTEMIS_NAMESPACE: &str = "Artemis";

pub struct MetricBuilder {
    metric_name: String,
    dimensions: Vec<Dimension>,
    value: f64,
}

// TODO: TxStatus type metrics => TxStatus(u32)
impl MetricBuilder {
    pub fn new(metric: CwMetrics) -> Self {
        match metric {
            CwMetrics::Balance(val) => Self {
                metric_name: format!("Bal-{}", val),
                dimensions: Vec::new(),
                value: 0.0,
            },
            _ => Self {
                metric_name: metric.into(),
                dimensions: Vec::new(),
                value: 1.0,
            },
        }
    }

    pub fn add_dimension(mut self, name: &str, value: &str) -> Self {
        self.dimensions
            .push(Dimension::builder().name(name).value(value).build());
        self
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn build(self) -> aws_sdk_cloudwatch::types::MetricDatum {
        aws_sdk_cloudwatch::types::MetricDatum::builder()
            .metric_name(self.metric_name)
            .value(self.value)
            .set_dimensions(Some(self.dimensions))
            .build()
    }
}

pub fn receipt_status_to_metric(status: u64) -> CwMetrics {
    match status {
        1 => CwMetrics::TxSucceeded,
        0 => CwMetrics::TxReverted,
        _ => CwMetrics::TxStatusUnknown,
    }
}
