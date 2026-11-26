use crate::kubernetes_api_objects::spec::{metric_identifier::MetricIdentifierView, metric_target::MetricTargetView};
use vstd::prelude::*;

verus! {

// ExternalMetricSourceView is the ghost type of ExternalMetricSource.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/external_metric_source.rs

pub struct ExternalMetricSourceView {
    pub metric: MetricIdentifierView,
    pub target: MetricTargetView,
}

impl ExternalMetricSourceView {
    pub open spec fn default() -> ExternalMetricSourceView {
        ExternalMetricSourceView {
            metric: MetricIdentifierView::default(),
            target: MetricTargetView::default(),
        }
    }
}

}
