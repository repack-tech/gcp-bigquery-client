//! A single row in the confusion matrix.
use crate::entry::Entry;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    /// The original label of this row.
    pub actual_label: Option<String>,
    /// Info describing predicted label distribution.
    pub entries: Option<Vec<Entry>>,
}
