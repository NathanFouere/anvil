use crate::kubernetes_api_objects::exec::{hpa_scaling_rules::*, resource::*};
use crate::kubernetes_api_objects::spec::horizontal_pod_autoscaler_behavior::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of HorizontalPodAutoscalerBehavior defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/horizontal_pod_autoscaler_behavior.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    HorizontalPodAutoscalerBehavior,
    k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
    HorizontalPodAutoscalerBehaviorView
);

impl HorizontalPodAutoscalerBehavior {
    #[verifier(external_body)]
    pub fn scale_down(&self) -> (scale_down: Option<HPAScalingRules>)
        ensures self@.scale_down == scale_down.deep_view(),
    {
        match &self.inner.scale_down {
            Some(r) => Some(HPAScalingRules::from_kube(r.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn scale_up(&self) -> (scale_up: Option<HPAScalingRules>)
        ensures self@.scale_up == scale_up.deep_view(),
    {
        match &self.inner.scale_up {
            Some(r) => Some(HPAScalingRules::from_kube(r.clone())),
            None => None,
        }
    }
}

}
