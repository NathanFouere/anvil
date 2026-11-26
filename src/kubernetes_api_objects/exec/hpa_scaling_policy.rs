use crate::kubernetes_api_objects::exec::resource::*;
use crate::kubernetes_api_objects::spec::hpa_scaling_policy::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of HPAScalingPolicy defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/hpa_scaling_policy.rs
// It is supposed to be used in exec controller code.
//
// More detailed information: https://kubernetes.io/docs/concepts/workloads/autoscaling/horizontal-pod-autoscale/.

implement_field_wrapper_type!(
    HPAScalingPolicy,
    k8s_openapi::api::autoscaling::v2::HPAScalingPolicy,
    HPAScalingPolicyView
);

impl HPAScalingPolicy {
    #[verifier(external_body)]
    pub fn period_seconds(&self) -> (period_seconds: i32)
        ensures period_seconds == self@.period_seconds,
    {
        self.inner.period_seconds
    }

    #[verifier(external_body)]
    pub fn type_(&self) -> (type_: String)
        ensures type_@ == self@.type_,
    {
        self.inner.type_.clone()
    }

    #[verifier(external_body)]
    pub fn value(&self) -> (value: i32)
        ensures value == self@.value,
    {
        self.inner.value
    }
}

}
