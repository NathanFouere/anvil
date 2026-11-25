use crate::kubernetes_api_objects::error::*;
use crate::kubernetes_api_objects::spec::prelude::*;
use crate::vstd_ext::string_view::StringView;
use vstd::prelude::*;

verus! {

pub struct MemcachedClusterView {
    pub metadata: ObjectMetaView,
    pub spec: MemcachedClusterSpecView,
    pub status: Option<MemcachedClusterStatusView>,
}

// TODO => copié de RabbitMqController, devrait reprendre la notion de condition
pub type MemcachedClusterStatusView = EmptyStatusView;

impl MemcachedClusterView {
    pub open spec fn with_metadata(self, metadata: ObjectMetaView) -> MemcachedClusterView {
        MemcachedClusterView {
            metadata: metadata,
            ..self
        }
    }

    pub open spec fn well_formed(self) -> bool {
        &&& self.metadata.well_formed_for_namespaced()
        &&& self.state_validation()
    }

    pub open spec fn controller_owner_ref(self) -> OwnerReferenceView {
        OwnerReferenceView {
            block_owner_deletion: Some(true),
            controller: Some(true),
            kind: Self::kind(),
            name: self.metadata.name->0,
            uid: self.metadata.uid->0,
        }
    }

    #[verifier(inline)]
    pub open spec fn _kind() -> Kind { Kind::CustomResourceKind("memcached"@) }

    #[verifier(inline)]
    pub open spec fn _state_validation(self) -> bool {
        &&& self.spec.replicas >= 0
        // TODO => à completer
    }

    #[verifier(inline)]
    pub open spec fn _transition_validation(self, old_obj: MemcachedClusterView) -> bool { true }
}

implement_resource_view_trait!(MemcachedClusterView, MemcachedClusterSpecView, MemcachedClusterSpecView::default(),
    Option<MemcachedClusterStatusView>, None, MemcachedClusterView::_kind(), _state_validation, _transition_validation);

impl CustomResourceView for MemcachedClusterView {
    proof fn kind_is_custom_resource() {}

    open spec fn spec_status_validation(obj_spec: Self::Spec, obj_status: Self::Status) -> bool {
        MemcachedClusterView {
            metadata: arbitrary(),
            spec: obj_spec,
            status: obj_status,
        }.state_validation()
    }

    proof fn validation_result_determined_by_spec_and_status() {}
}

pub struct MemcachedClusterSpecView {
    pub replicas: int,
    pub image: StringView,
    pub labels: Map<StringView, StringView>,
    pub annotations: Map<StringView, StringView>,
    // TODO => Compléter
}

impl MemcachedClusterSpecView {
    pub open spec fn default() -> MemcachedClusterSpecView {
        MemcachedClusterSpecView {
            replicas: 0,
            image: ""@,
            labels: Map::empty(),
            annotations: Map::empty(),
        }
    }
}

}
