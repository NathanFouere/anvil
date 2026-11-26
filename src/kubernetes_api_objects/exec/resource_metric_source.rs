use crate::kubernetes_api_objects::exec::{metric_target::*, resource::*};
use crate::kubernetes_api_objects::spec::resource_metric_source::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of ResourceMetricSource defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/resource_metric_source.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    ResourceMetricSource,
    k8s_openapi::api::autoscaling::v2::ResourceMetricSource,
    ResourceMetricSourceView
);

impl ResourceMetricSource {
    #[verifier(external_body)]
    pub fn name(&self) -> (name: String)
        ensures name@ == self@.name,
    {
        self.inner.name.clone()
    }

    #[verifier(external_body)]
    pub fn target(&self) -> (target: MetricTarget)
        ensures target@ == self@.target,
    {
        MetricTarget::from_kube(self.inner.target.clone())
    }
}

}
