use crate::{kubernetes_api_objects::spec::label_selector::LabelSelectorView, vstd_ext::string_view::*};
use vstd::prelude::*;

verus! {

// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_identifier.rs
// MetricIdentifierView is the ghost type of MetricIdentifier.

pub struct MetricIdentifierView {
    pub name: StringView,
    pub selector: Option<LabelSelectorView>,
}

impl MetricIdentifierView {
    pub open spec fn default() -> MetricIdentifierView {
        MetricIdentifierView {
            name: ""@,
            selector: None,
        }
    }
}

}
