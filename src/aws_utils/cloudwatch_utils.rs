use std::{future::Future, pin::Pin, sync::Arc};

use aws_sdk_cloudwatch::Client as CloudWatchClient;
use aws_sdk_cloudwatch::{
    config::http::HttpResponse,
    error::SdkError,
    operation::put_metric_data::{PutMetricDataError, PutMetricDataOutput},
    types::Dimension,
};

/// Constants for dimension names and values
pub const SERVICE_DIMENSION: &str = "Service";
pub const ROUTER02: &str = "Router02";
pub const PRIORITY_EXECUTOR: &str = "PriorityExecutor";
pub const V2_EXECUTOR: &str = "V2Executor";
pub const V3_EXECUTOR: &str = "V3Executor";

/// Constants for metric names
pub const ROUTING_MS: &str = "RoutingMs";
pub const TX_SUCCEEDED_METRIC: &str = "TransactionSucceeded";
pub const TX_REVERTED_METRIC: &str = "TransactionReverted";
pub const TX_SUBMITTED_METRIC: &str = "TransactionSubmitted";
pub const TX_STATUS_UNKNOWN_METRIC: &str = "TransactionStatusUnknown";
pub const LATEST_BLOCK: &str = "LatestBlock";
pub const EXECUTION_ATTEMPTED_METRIC: &str = "ExecutionAttempted";
pub const EXECUTION_SKIPPED_ALREADY_FILLED_METRIC: &str = "ExecutionSkippedAlreadyFilled";
pub const EXECUTION_SKIPPED_PAST_DEADLINE_METRIC: &str = "ExecutionSkippedPastDeadline";
pub const UNPROFITABLE_METRIC: &str = "Unprofitable";
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
    V3Executor,
    Router02,
}
impl From<DimensionValue> for String {
    fn from(value: DimensionValue) -> Self {
        match value {
            DimensionValue::PriorityExecutor => PRIORITY_EXECUTOR.to_string(),
            DimensionValue::V2Executor => V2_EXECUTOR.to_string(),
            DimensionValue::V3Executor => V3_EXECUTOR.to_string(),
            DimensionValue::Router02 => ROUTER02.to_string(),
        }
    }
}

impl AsRef<str> for DimensionValue {
    fn as_ref(&self) -> &str {
        match self {
            DimensionValue::PriorityExecutor => PRIORITY_EXECUTOR,
            DimensionValue::V2Executor => V2_EXECUTOR,
            DimensionValue::V3Executor => V3_EXECUTOR,
            DimensionValue::Router02 => ROUTER02,
        }
    }
}

pub enum CwMetrics {
    RoutingMs,
    Unprofitable,
    ExecutionAttempted,
    ExecutionSkippedAlreadyFilled,
    ExecutionSkippedPastDeadline,
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
            CwMetrics::RoutingMs => ROUTING_MS.to_string(),
            CwMetrics::Unprofitable => UNPROFITABLE_METRIC.to_string(),
            CwMetrics::ExecutionAttempted => EXECUTION_ATTEMPTED_METRIC.to_string(),
            CwMetrics::ExecutionSkippedAlreadyFilled => {
                EXECUTION_SKIPPED_ALREADY_FILLED_METRIC.to_string()
            }
            CwMetrics::ExecutionSkippedPastDeadline => {
                EXECUTION_SKIPPED_PAST_DEADLINE_METRIC.to_string()
            }
            CwMetrics::TxSucceeded => TX_SUCCEEDED_METRIC.to_string(),
            CwMetrics::TxReverted => TX_REVERTED_METRIC.to_string(),
            CwMetrics::TxSubmitted => TX_SUBMITTED_METRIC.to_string(),
            CwMetrics::TxStatusUnknown => TX_STATUS_UNKNOWN_METRIC.to_string(),
            CwMetrics::Balance(val) => format!("Bal-{}", val),
            CwMetrics::LatestBlock => LATEST_BLOCK.to_string(),
        }
    }
}

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

pub fn build_metric_future(
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
    dimension_value: DimensionValue,
    metric: CwMetrics,
    value: f64,
) -> Option<
    Pin<
        Box<
            impl Future<
                    Output = Result<
                        PutMetricDataOutput,
                        SdkError<PutMetricDataError, HttpResponse>,
                    >,
                > + Send
                + 'static,
        >,
    >,
> {
    cloudwatch_client.map(|client| {
        Box::pin(async move {
            client
                .put_metric_data()
                .namespace(ARTEMIS_NAMESPACE)
                .metric_data(
                    MetricBuilder::new(metric)
                        .add_dimension(DimensionName::Service.as_ref(), dimension_value.as_ref())
                        .with_value(value)
                        .build(),
                )
                .send()
                .await
        })
    })
}

#[macro_export]
macro_rules! send_metric_with_order_hash {
    ($order_hash: expr, $future: expr) => {
        let hash = Arc::clone($order_hash);
        tokio::spawn(async move {
            if let Err(e) = $future.await {
                warn!("{} - error sending metric: {:?}", hash, e);
            }
        })
    };
}

#[macro_export]
macro_rules! send_metric {
    ($future: expr) => {
        tokio::spawn(async move {
            if let Err(e) = $future.await {
                warn!("error sending metric: {:?}", e);
            }
        })
    };
}
