use crate::kubernetes_api_objects::spec::quantity::QuantityView;
use vstd::prelude::*;

verus! {

// MetricValueStatusView is the ghost type of MetricValueStatus.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_value_status.rs

pub struct MetricValueStatusView {
    pub average_utilization: Option<int>,
    pub average_value: Option<QuantityView>,
    pub value: Option<QuantityView>,
}

impl MetricValueStatusView {
    pub open spec fn default() -> MetricValueStatusView {
        MetricValueStatusView {
            average_utilization: None,
            average_value: None,
            value: None,
        }
    }
}

}
