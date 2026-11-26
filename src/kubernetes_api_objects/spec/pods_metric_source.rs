use crate::kubernetes_api_objects::spec::{metric_identifier::MetricIdentifierView, metric_target::MetricTargetView};
use vstd::prelude::*;

verus! {

// PodsMetricSourceView is the ghost type of PodsMetricSource.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/pods_metric_source.rs

pub struct PodsMetricSourceView {
    pub metric: MetricIdentifierView,
    pub target: MetricTargetView,
}

impl PodsMetricSourceView {
    pub open spec fn default() -> PodsMetricSourceView {
        PodsMetricSourceView {
            metric: MetricIdentifierView::default(),
            target: MetricTargetView::default(),
        }
    }
}

}
