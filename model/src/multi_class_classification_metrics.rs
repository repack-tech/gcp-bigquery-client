//! Evaluation metrics for multi-class classification/classifier models.
use crate::aggregate_classification_metrics::AggregateClassificationMetrics;
use crate::confusion_matrix::ConfusionMatrix;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiClassClassificationMetrics {
    /// Aggregate classification metrics.
    pub aggregate_classification_metrics: Option<AggregateClassificationMetrics>,
    /// Confusion matrix at different thresholds.
    pub confusion_matrix_list: Option<Vec<ConfusionMatrix>>,
}
