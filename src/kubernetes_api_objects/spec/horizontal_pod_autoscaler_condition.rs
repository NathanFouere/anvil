use crate::{kubernetes_api_objects::spec::time::TimeView, vstd_ext::string_view::*};
use vstd::prelude::*;

verus! {

// HorizontalPodAutoscalerConditionView is the ghost type of HorizontalPodAutoscalerCondition.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/horizontal_pod_autoscaler_condition.rs

pub struct HorizontalPodAutoscalerConditionView {
    pub last_transition_time: Option<TimeView>,
    pub message: Option<StringView>,
    pub reason: Option<StringView>,
    pub status: StringView,
    pub type_: StringView,
}

impl HorizontalPodAutoscalerConditionView {
    pub open spec fn default() -> HorizontalPodAutoscalerConditionView {
        HorizontalPodAutoscalerConditionView {
            last_transition_time: None,
            message: None,
            reason: None,
            status: ""@,
            type_: ""@,
        }
    }
}

}
