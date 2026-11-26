use crate::kubernetes_api_objects::exec::{
    container_resource_metric_source::*, external_metric_source::*, object_metric_source::*,
    pods_metric_source::*, resource::*, resource_metric_source::*,
};
use crate::kubernetes_api_objects::spec::metric_spec::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of MetricSpec defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/metric_spec.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    MetricSpec,
    k8s_openapi::api::autoscaling::v2::MetricSpec,
    MetricSpecView
);

impl MetricSpec {
    #[verifier(external_body)]
    pub fn container_resource(&self) -> (container_resource: Option<ContainerResourceMetricSource>)
        ensures self@.container_resource == container_resource.deep_view(),
    {
        match &self.inner.container_resource {
            Some(s) => Some(ContainerResourceMetricSource::from_kube(s.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn external(&self) -> (external: Option<ExternalMetricSource>)
        ensures self@.external == external.deep_view(),
    {
        match &self.inner.external {
            Some(s) => Some(ExternalMetricSource::from_kube(s.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn object(&self) -> (object: Option<ObjectMetricSource>)
        ensures self@.object == object.deep_view(),
    {
        match &self.inner.object {
            Some(s) => Some(ObjectMetricSource::from_kube(s.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn pods(&self) -> (pods: Option<PodsMetricSource>)
        ensures self@.pods == pods.deep_view(),
    {
        match &self.inner.pods {
            Some(s) => Some(PodsMetricSource::from_kube(s.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn resource(&self) -> (resource: Option<ResourceMetricSource>)
        ensures self@.resource == resource.deep_view(),
    {
        match &self.inner.resource {
            Some(s) => Some(ResourceMetricSource::from_kube(s.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn type_(&self) -> (type_: String)
        ensures type_@ == self@.type_,
    {
        self.inner.type_.clone()
    }
}

}
