use crate::kubernetes_api_objects::exec::{hpa_scaling_policy::*, resource::*};
use crate::kubernetes_api_objects::spec::hpa_scaling_rules::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of HPAScalingRules defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/hpa_scaling_rules.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    HPAScalingRules,
    k8s_openapi::api::autoscaling::v2::HPAScalingRules,
    HPAScalingRulesView
);

impl HPAScalingRules {
    #[verifier(external_body)]
    pub fn policies(&self) -> (policies: Option<Vec<HPAScalingPolicy>>)
        ensures self@.policies == policies.deep_view(),
    {
        match &self.inner.policies {
            Some(p) => Some(p.into_iter().map(|item| HPAScalingPolicy::from_kube(item.clone())).collect()),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn select_policy(&self) -> (select_policy: Option<String>)
        ensures self@.select_policy == opt_string_to_view(&select_policy),
    {
        self.inner.select_policy.clone()
    }

    #[verifier(external_body)]
    pub fn stabilization_window_seconds(&self) -> (stabilization_window_seconds: Option<i32>)
        ensures
            self@.stabilization_window_seconds is Some == stabilization_window_seconds is Some,
            stabilization_window_seconds is Some ==> stabilization_window_seconds->0 as int == self@.stabilization_window_seconds->0,
    {
        self.inner.stabilization_window_seconds
    }
}

}
