use crate::kubernetes_api_objects::exec::{quantity::*, resource::*};
use crate::kubernetes_api_objects::spec::metric_value_status::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of MetricValueStatus defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_value_status.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    MetricValueStatus,
    k8s_openapi::api::autoscaling::v2::MetricValueStatus,
    MetricValueStatusView
);

impl MetricValueStatus {
    #[verifier(external_body)]
    pub fn average_utilization(&self) -> (average_utilization: Option<i32>)
        ensures
            self@.average_utilization is Some == average_utilization is Some,
            average_utilization is Some ==> average_utilization->0 as int == self@.average_utilization->0,
    {
        self.inner.average_utilization
    }

    #[verifier(external_body)]
    pub fn average_value(&self) -> (average_value: Option<Quantity>)
        ensures self@.average_value == average_value.deep_view(),
    {
        match &self.inner.average_value {
            Some(q) => Some(Quantity::from_kube(q.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn value(&self) -> (value: Option<Quantity>)
        ensures self@.value == value.deep_view(),
    {
        match &self.inner.value {
            Some(q) => Some(Quantity::from_kube(q.clone())),
            None => None,
        }
    }
}

}
