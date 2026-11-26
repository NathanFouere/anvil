use crate::{kubernetes_api_objects::spec::metric_target::MetricTargetView, vstd_ext::string_view::*};
use vstd::prelude::*;

verus! {

// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/container_resource_metric_source.rs
// ContainerResourceMetricSourceView is the ghost type of ContainerResourceMetricSource.

pub struct ContainerResourceMetricSourceView {
    pub container: StringView,
    pub name: StringView,
    pub target: MetricTargetView,
}

impl ContainerResourceMetricSourceView {
    pub open spec fn default() -> ContainerResourceMetricSourceView {
        ContainerResourceMetricSourceView {
            container: ""@,
            name: ""@,
            target: MetricTargetView::default(),
        }
    }
}

}
