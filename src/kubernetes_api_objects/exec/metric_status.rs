use crate::kubernetes_api_objects::exec::{resource::*, resource_metric_status::*};
use crate::kubernetes_api_objects::spec::metric_status::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of MetricStatus defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_status.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    MetricStatus,
    k8s_openapi::api::autoscaling::v2::MetricStatus,
    MetricStatusView
);

impl MetricStatus {
    #[verifier(external_body)]
    pub fn resource(&self) -> (resource: Option<ResourceMetricStatus>)
        ensures self@.resource == resource.deep_view(),
    {
        match &self.inner.resource {
            Some(r) => Some(ResourceMetricStatus::from_kube(r.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn type_(&self) -> (type_: String)
        ensures type_@ == self@.type_,
    {
        self.inner.type_.clone()
    }
}

}
