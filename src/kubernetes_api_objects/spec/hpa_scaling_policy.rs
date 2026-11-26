use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// HPAScalingPolicyView is the ghost type of HPAScalingPolicy.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/hpa_scaling_policy.rs

pub struct HPAScalingPolicyView {
    pub period_seconds: int,
    pub type_: StringView,
    pub value: int,
}

impl HPAScalingPolicyView {
    pub open spec fn default() -> HPAScalingPolicyView {
        HPAScalingPolicyView {
            period_seconds: 0,
            type_: ""@,
            value: 0,
        }
    }

    pub open spec fn with_period_seconds(self, period_seconds: int) -> HPAScalingPolicyView {
        HPAScalingPolicyView { period_seconds: period_seconds, ..self }
    }

    pub open spec fn with_type(self, type_: StringView) -> HPAScalingPolicyView {
        HPAScalingPolicyView { type_: type_, ..self }
    }

    pub open spec fn with_value(self, value: int) -> HPAScalingPolicyView {
        HPAScalingPolicyView { value: value, ..self }
    }
}

}
