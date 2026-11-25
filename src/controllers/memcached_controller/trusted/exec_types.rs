use crate::kubernetes_api_objects::error::UnmarshalError;
use crate::kubernetes_api_objects::exec::{api_resource::*, prelude::*, resource::*};
use crate::kubernetes_api_objects::spec::resource::*;
use crate::memcached_controller::trusted::spec_types;
use crate::vstd_ext::{string_map::*, string_view::*};
use kube::Resource;
use vstd::prelude::*;

verus! {

implement_custom_object_wrapper_type!(
    MemcachedCluster,
    crate::crds::MemcachedCluster,
    spec_types::MemcachedClusterView
);

impl MemcachedCluster {
    #[verifier(external_body)]
    pub fn spec(&self) -> (spec: MemcachedClusterSpec)
        ensures spec@ == self@.spec,
    {
        MemcachedClusterSpec { inner: self.inner.spec.clone() }
    }

    #[verifier(external_body)]
    pub fn well_formed(&self) -> (b: bool)
        ensures b == self@.well_formed(),
    {
        self.metadata().well_formed_for_namespaced()
        && self.state_validation()
    }

    pub fn state_validation(&self) -> (res: bool)
        ensures res == self@.state_validation(),
    {
        self.spec().replicas() >= 0
    }
}

#[verifier(external_body)]
pub struct MemcachedClusterSpec {
    inner: crate::crds::MemcachedClusterSpec,
}

implement_view_trait!(MemcachedClusterSpec, spec_types::MemcachedClusterSpecView);

impl MemcachedClusterSpec {
    #[verifier(external_body)]
    pub fn replicas(&self) -> (replicas: i32)
        ensures replicas as int == self@.replicas,
    {
        self.inner.replicas
    }

    #[verifier(external_body)]
    pub fn image(&self) -> (image: String)
        ensures image@ == self@.image,
    {
        self.inner.image.clone()
    }

    #[verifier(external_body)]
    pub fn labels(&self) -> (labels: StringMap)
        ensures labels@ == self@.labels,
    {
        StringMap::from_rust_map(self.inner.labels.clone())
    }

    #[verifier(external_body)]
    pub fn annotations(&self) -> (annotations: StringMap)
        ensures annotations@ == self@.annotations,
    {
        StringMap::from_rust_map(self.inner.annotations.clone())
    }
}

}
