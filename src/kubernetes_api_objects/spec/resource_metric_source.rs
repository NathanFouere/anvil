use crate::{kubernetes_api_objects::spec::metric_target::MetricTargetView, vstd_ext::string_view::*};
use vstd::prelude::*;

verus! {

// ResourceMetricSourceView is the ghost type of ResourceMetricSource.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/resource_metric_source.rs

pub struct ResourceMetricSourceView {
    pub name: StringView,
    pub target: MetricTargetView,
}

impl ResourceMetricSourceView {
    pub open spec fn default() -> ResourceMetricSourceView {
        ResourceMetricSourceView {
            name: ""@,
            target: MetricTargetView::default(),
        }
    }
}

}
