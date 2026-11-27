use crate::{kubernetes_api_objects::spec::resource_metric_status::ResourceMetricStatusView, vstd_ext::string_view::*};
use vstd::prelude::*;

verus! {

// MetricStatusView is the ghost type of MetricStatus.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_status.rs

pub struct MetricStatusView {
    // TODO: ici on représente qu'un sous ensemble des champs, à voir si il en faut plus
    pub resource: Option<ResourceMetricStatusView>,
    pub type_: StringView,
}

impl MetricStatusView {
    pub open spec fn default() -> MetricStatusView {
        MetricStatusView {
            resource: None,
            type_: ""@,
        }
    }
}

}
