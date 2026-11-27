use crate::{kubernetes_api_objects::spec::metric_value_status::MetricValueStatusView, vstd_ext::string_view::*};
use vstd::prelude::*;

verus! {

// ResourceMetricStatusView is the ghost type of ResourceMetricStatus.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/resource_metric_status.rs

pub struct ResourceMetricStatusView {
    pub current: MetricValueStatusView,
    pub name: StringView,
}

impl ResourceMetricStatusView {
    pub open spec fn default() -> ResourceMetricStatusView {
        ResourceMetricStatusView {
            current: MetricValueStatusView::default(),
            name: ""@,
        }
    }
}

}
