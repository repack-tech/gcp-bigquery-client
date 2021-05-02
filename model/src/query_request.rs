use crate::connection_property::ConnectionProperty;
use crate::dataset_reference::DatasetReference;
use crate::query_parameter::QueryParameter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    /// Connection properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<Vec<ConnectionProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_dataset: Option<DatasetReference>,
    /// [Optional] If set to true, BigQuery doesn't run the job. Instead, if the query is valid, BigQuery returns statistics about the job such as how many bytes would be processed. If the query is invalid, an error returns. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The resource type of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// The geographic location where the job should run. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// [Optional] The maximum number of rows of data to return per page of results. Setting this flag to a small value such as 1000 and then paging through results might improve reliability when the query result set is large. In addition to this limit, responses are also limited to 10 MB. By default, there is no maximum row count, and only the byte limit applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// [Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bytes_billed: Option<String>,
    /// Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_mode: Option<String>,
    /// [Deprecated] This property is deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_nulls: Option<bool>,
    /// [Required] A query string, following the BigQuery query syntax, of the query to execute. Example: \"SELECT count(f1) FROM [myProjectId:myDatasetId.myTableId]\".
    pub query: String,
    /// Query parameters for Standard SQL queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<QueryParameter>>,
    /// A unique user provided identifier to ensure idempotent behavior for queries. Note that this is different from the job_id. It has the following properties: 1. It is case-sensitive, limited to up to 36 ASCII characters. A UUID is recommended. 2. Read only queries can ignore this token since they are nullipotent by definition. 3. For the purposes of idempotency ensured by the request_id, a request is considered duplicate of another only if they have the same request_id and are actually duplicates. When determining whether a request is a duplicate of the previous request, all parameters in the request that may affect the behavior are considered. For example, query, connection_properties, query_parameters, use_legacy_sql are parameters that affect the result and are considered when determining whether a request is a duplicate, but properties like timeout_ms don't affect the result and are thus not considered. Dry run query requests are never considered duplicate of another request. 4. When a duplicate mutating query request is detected, it returns: a. the results of the mutation if it completes successfully within the timeout. b. the running operation if it is still in progress at the end of the timeout. 5. Its lifetime is limited to 15 minutes. In other words, if two requests are sent with the same request_id, but more than 15 minutes apart, idempotency is not guaranteed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// [Optional] How long to wait for the query to complete, in milliseconds, before the request times out and returns. Note that this is only a timeout for the request, not the query. If the query takes longer to run than the timeout value, the call returns without any results and with the 'jobComplete' flag set to false. You can call GetQueryResults() to wait for the query to complete and read the results. The default value is 10000 milliseconds (10 seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i32>,
    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
    pub use_legacy_sql: bool,
    /// [Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. The default value is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_query_cache: Option<bool>,
}

impl QueryRequest {
    pub fn new(sql_query: impl Into<String>) -> Self {
        Self {
            connection_properties: None,
            default_dataset: None,
            dry_run: None,
            kind: None,
            labels: None,
            location: None,
            max_results: None,
            maximum_bytes_billed: None,
            parameter_mode: None,
            preserve_nulls: None,
            query: sql_query.into(),
            query_parameters: None,
            request_id: None,
            timeout_ms: None,
            use_legacy_sql: false, // force standard SQL by default
            use_query_cache: None,
        }
    }
}
