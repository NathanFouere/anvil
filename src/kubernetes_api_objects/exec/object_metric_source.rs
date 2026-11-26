use crate::kubernetes_api_objects::exec::{
    cross_version_objet_reference::*, metric_identifier::*, metric_target::*, resource::*,
};
use crate::kubernetes_api_objects::spec::object_metric_source::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of ObjectMetricSource defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/object_metric_source.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    ObjectMetricSource,
    k8s_openapi::api::autoscaling::v2::ObjectMetricSource,
    ObjectMetricSourceView
);

impl ObjectMetricSource {
    #[verifier(external_body)]
    pub fn described_object(&self) -> (described_object: CrossVersionObjectReference)
        ensures described_object@ == self@.described_object,
    {
        CrossVersionObjectReference::from_kube(self.inner.described_object.clone())
    }

    #[verifier(external_body)]
    pub fn metric(&self) -> (metric: MetricIdentifier)
        ensures metric@ == self@.metric,
    {
        MetricIdentifier::from_kube(self.inner.metric.clone())
    }

    #[verifier(external_body)]
    pub fn target(&self) -> (target: MetricTarget)
        ensures target@ == self@.target,
    {
        MetricTarget::from_kube(self.inner.target.clone())
    }
}

}
