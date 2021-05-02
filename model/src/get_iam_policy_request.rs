use crate::get_policy_options::GetPolicyOptions;
use serde::{Deserialize, Serialize};

/// GetIamPolicyRequest : Request message for `GetIamPolicy` method.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIamPolicyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GetPolicyOptions>,
}
