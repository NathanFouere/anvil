use crate::kubernetes_api_objects::exec::{label_selector::*, resource::*};
use crate::kubernetes_api_objects::spec::metric_identifier::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of MetricIdentifier defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_identifier.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    MetricIdentifier,
    k8s_openapi::api::autoscaling::v2::MetricIdentifier,
    MetricIdentifierView
);

impl MetricIdentifier {
    #[verifier(external_body)]
    pub fn name(&self) -> (name: String)
        ensures name@ == self@.name,
    {
        self.inner.name.clone()
    }

    #[verifier(external_body)]
    pub fn selector(&self) -> (selector: Option<LabelSelector>)
        ensures self@.selector == selector.deep_view(),
    {
        match &self.inner.selector {
            Some(s) => Some(LabelSelector::from_kube(s.clone())),
            None => None,
        }
    }
}

}
