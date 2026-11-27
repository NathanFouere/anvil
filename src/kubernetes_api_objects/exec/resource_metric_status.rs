use crate::kubernetes_api_objects::exec::{metric_value_status::*, resource::*};
use crate::kubernetes_api_objects::spec::resource_metric_status::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of ResourceMetricStatus defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/resource_metric_status.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    ResourceMetricStatus,
    k8s_openapi::api::autoscaling::v2::ResourceMetricStatus,
    ResourceMetricStatusView
);

impl ResourceMetricStatus {
    #[verifier(external_body)]
    pub fn current(&self) -> (current: MetricValueStatus)
        ensures current@ == self@.current,
    {
        MetricValueStatus::from_kube(self.inner.current.clone())
    }

    #[verifier(external_body)]
    pub fn name(&self) -> (name: String)
        ensures name@ == self@.name,
    {
        self.inner.name.clone()
    }
}

}
