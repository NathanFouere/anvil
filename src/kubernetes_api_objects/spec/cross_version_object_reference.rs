use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// CrossVersionObjectReferenceView is the ghost type of CrossVersionObjectReference.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/cross_version_object_reference.rs

pub struct CrossVersionObjectReferenceView {
    pub api_version: Option<StringView>,
    pub kind: StringView,
    pub name: StringView,
}

impl CrossVersionObjectReferenceView {
    pub open spec fn default() -> CrossVersionObjectReferenceView {
        CrossVersionObjectReferenceView {
            api_version: None,
            kind: ""@,
            name: ""@,
        }
    }
}

}
