use crate::kubernetes_api_objects::exec::{metric_identifier::*, metric_target::*, resource::*};
use crate::kubernetes_api_objects::spec::external_metric_source::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of ExternalMetricSource defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/external_metric_source.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    ExternalMetricSource,
    k8s_openapi::api::autoscaling::v2::ExternalMetricSource,
    ExternalMetricSourceView
);

impl ExternalMetricSource {
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
