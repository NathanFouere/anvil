use crate::kubernetes_api_objects::exec::resource::*;
use crate::kubernetes_api_objects::spec::cross_version_object_reference::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of CrossVersionObjectReference defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/cross_version_object_reference.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    CrossVersionObjectReference,
    k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference,
    CrossVersionObjectReferenceView
);

impl CrossVersionObjectReference {
    #[verifier(external_body)]
    pub fn api_version(&self) -> (api_version: Option<String>)
        ensures self@.api_version == opt_string_to_view(&api_version),
    {
        self.inner.api_version.clone()
    }

    #[verifier(external_body)]
    pub fn kind(&self) -> (kind: String)
        ensures kind@ == self@.kind,
    {
        self.inner.kind.clone()
    }

    #[verifier(external_body)]
    pub fn name(&self) -> (name: String)
        ensures name@ == self@.name,
    {
        self.inner.name.clone()
    }
}

}
