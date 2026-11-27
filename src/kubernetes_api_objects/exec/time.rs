use crate::kubernetes_api_objects::exec::resource::*;
use crate::kubernetes_api_objects::spec::time::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of Time defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/apimachinery/pkg/apis/meta/v1/time.rs
// It is supposed to be used in exec controller code.

#[verifier(external_body)]
pub struct Time {
    inner: k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
}

}

// Time supporte pas "default"
implement_view_trait!(Time, TimeView);
implement_deep_view_trait!(Time, TimeView);
implement_clone_trait!(Time);
implement_resource_wrapper_trait!(Time, k8s_openapi::apimachinery::pkg::apis::meta::v1::Time);
implement_eq!(Time); // TODO => à confirmer, voir comment k8s stocker le temps

verus! {

impl Time {
    #[verifier(external_body)]
    pub fn timestamp(&self) -> (timestamp: i64)
        ensures timestamp == self@.timestamp,
    {
        self.inner.0.timestamp()
    }
}

}
