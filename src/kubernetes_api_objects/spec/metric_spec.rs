use crate::kubernetes_api_objects::spec::{
    container_resource_metric_source::ContainerResourceMetricSourceView,
    external_metric_source::ExternalMetricSourceView,
    object_metric_source::ObjectMetricSourceView,
    pods_metric_source::PodsMetricSourceView,
    resource_metric_source::ResourceMetricSourceView,
};
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// MetricSpecView is the ghost type of MetricSpec.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_spec.rs

pub struct MetricSpecView {
    pub container_resource: Option<ContainerResourceMetricSourceView>,
    pub external: Option<ExternalMetricSourceView>,
    pub object: Option<ObjectMetricSourceView>,
    pub pods: Option<PodsMetricSourceView>,
    pub resource: Option<ResourceMetricSourceView>,
    pub type_: StringView,
}

impl MetricSpecView {
    pub open spec fn default() -> MetricSpecView {
        MetricSpecView {
            container_resource: None,
            external: None,
            object: None,
            pods: None,
            resource: None,
            type_: ""@,
        }
    }
}

}
