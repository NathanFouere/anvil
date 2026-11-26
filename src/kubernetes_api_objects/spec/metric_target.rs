use crate::kubernetes_api_objects::spec::quantity::QuantityView;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// MetricTargetView is the ghost type of MetricTarget.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_target.rs

pub struct MetricTargetView {
    pub average_utilization: Option<int>,
    pub average_value: Option<QuantityView>,
    pub type_: StringView,
    pub value: Option<QuantityView>,
}

impl MetricTargetView {
    pub open spec fn default() -> MetricTargetView {
        MetricTargetView {
            average_utilization: None,
            average_value: None,
            type_: ""@,
            value: None,
        }
    }
}

}
