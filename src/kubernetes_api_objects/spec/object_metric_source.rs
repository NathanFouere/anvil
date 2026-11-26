use crate::kubernetes_api_objects::spec::{
    cross_version_objet_reference::CrossVersionObjectReferenceView,
    metric_identifier::MetricIdentifierView,
    metric_target::MetricTargetView,
};
use vstd::prelude::*;

verus! {

// ObjectMetricSourceView is the ghost type of ObjectMetricSource.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/object_metric_source.rs

pub struct ObjectMetricSourceView {
    pub described_object: CrossVersionObjectReferenceView,
    pub metric: MetricIdentifierView,
    pub target: MetricTargetView,
}

impl ObjectMetricSourceView {
    pub open spec fn default() -> ObjectMetricSourceView {
        ObjectMetricSourceView {
            described_object: CrossVersionObjectReferenceView::default(),
            metric: MetricIdentifierView::default(),
            target: MetricTargetView::default(),
        }
    }
}

}
