use crate::kubernetes_api_objects::error::*;
use crate::kubernetes_api_objects::spec::cross_version_object_reference::CrossVersionObjectReferenceView;
use crate::kubernetes_api_objects::spec::horizontal_pod_autoscaler_behavior::HorizontalPodAutoscalerBehaviorView;
use crate::kubernetes_api_objects::spec::horizontal_pod_autoscaler_condition::HorizontalPodAutoscalerConditionView;
use crate::kubernetes_api_objects::spec::metric_spec::MetricSpecView;
use crate::kubernetes_api_objects::spec::metric_status::MetricStatusView;
use crate::kubernetes_api_objects::spec::time::TimeView;
use crate::kubernetes_api_objects::spec::{label_selector::*, pod_template_spec::*, prelude::*};
use crate::vreplicaset_controller::trusted::spec_types::VReplicaSetView;
use vstd::prelude::*;

verus! {

pub struct VHorizontalPodAutoscalerView {
    pub metadata: ObjectMetaView,
    pub spec: VHorizontalPodAutoscalerSpecView,
    pub status: Option<VHorizontalPodAutoscalerStatusView>,
}

impl VHorizontalPodAutoscalerView {
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

    pub open spec fn with_metadata(self, metadata: ObjectMetaView) -> VHorizontalPodAutoscalerView {
        VHorizontalPodAutoscalerView { metadata: metadata, ..self }
    }

    pub open spec fn with_spec(
        self,
        spec: VHorizontalPodAutoscalerSpecView,
    ) -> VHorizontalPodAutoscalerView {
        VHorizontalPodAutoscalerView { spec: spec, ..self }
    }

    pub open spec fn with_status(
        self,
        status: VHorizontalPodAutoscalerStatusView,
    ) -> VHorizontalPodAutoscalerView {
        VHorizontalPodAutoscalerView { status: Some(status), ..self }
    }

    #[verifier(inline)]
    pub open spec fn _kind() -> Kind {
        Kind::CustomResourceKind("vhorizontalpodautoscaler"@)
    }

    // TODO: keep it consistent with k8s's HPA
    #[verifier(inline)]
    pub open spec fn _state_validation(self) -> bool {
        true
    }

    // TODO => vérifier que aussi simple que dans ReplicaSet, sinn, se baser sur controller RabbitMq
    #[verifier(inline)]
    pub open spec fn _transition_validation(self, old_obj: VHorizontalPodAutoscalerView) -> bool {
        true
    }
}

implement_resource_view_trait!(VHorizontalPodAutoscalerView, VHorizontalPodAutoscalerSpecView, VHorizontalPodAutoscalerSpecView::default(),
        Option<VHorizontalPodAutoscalerStatusView>, None, VHorizontalPodAutoscalerView::_kind(), _state_validation, _transition_validation);

impl CustomResourceView for VHorizontalPodAutoscalerView {
    proof fn kind_is_custom_resource() {
    }

    open spec fn spec_status_validation(obj_spec: Self::Spec, obj_status: Self::Status) -> bool {
        VHorizontalPodAutoscalerView {
            metadata: arbitrary(),
            spec: obj_spec,
            status: obj_status,
        }.state_validation()
    }

    proof fn validation_result_determined_by_spec_and_status() {
    }
}

pub struct VHorizontalPodAutoscalerSpecView {
    pub behavior: Option<HorizontalPodAutoscalerBehaviorView>,
    pub max_replicas: int,
    pub metrics: Option<MetricSpecView>,
    pub min_replicas: Option<int>,
    pub scale_target_ref: CrossVersionObjectReferenceView,
}

impl VHorizontalPodAutoscalerSpecView {
    pub open spec fn default() -> VHorizontalPodAutoscalerSpecView {
        VHorizontalPodAutoscalerSpecView {
            behavior: None,
            max_replicas: 0,
            metrics: None,
            min_replicas: None,
            scale_target_ref: CrossVersionObjectReferenceView::default(),
        }
    }

    pub open spec fn with_behavior(
        self,
        behavior: HorizontalPodAutoscalerBehaviorView,
    ) -> VHorizontalPodAutoscalerSpecView {
        VHorizontalPodAutoscalerSpecView { behavior: Some(behavior), ..self }
    }

    pub open spec fn with_max_replicas(
        self,
        max_replicas: int,
    ) -> VHorizontalPodAutoscalerSpecView {
        VHorizontalPodAutoscalerSpecView { max_replicas: max_replicas, ..self }
    }

    pub open spec fn with_metrics(
        self,
        metrics: MetricSpecView,
    ) -> VHorizontalPodAutoscalerSpecView {
        VHorizontalPodAutoscalerSpecView { metrics: Some(metrics), ..self }
    }

    pub open spec fn with_min_replicas(
        self,
        min_replicas: int,
    ) -> VHorizontalPodAutoscalerSpecView {
        VHorizontalPodAutoscalerSpecView { min_replicas: Some(min_replicas), ..self }
    }
}

pub struct VHorizontalPodAutoscalerStatusView {
    pub conditions: Option<HorizontalPodAutoscalerConditionView>,
    pub current_metrics: Option<MetricStatusView>,
    pub current_replicas: Option<int>,
    pub desired_replicas: int,
    pub last_scale_time: Option<TimeView>,
}

impl VHorizontalPodAutoscalerStatusView {
    pub open spec fn default() -> VHorizontalPodAutoscalerStatusView {
        VHorizontalPodAutoscalerStatusView {
            conditions: None,
            current_metrics: None,
            current_replicas: None,
            desired_replicas: 0,
            last_scale_time: None,
        }
    }

    pub open spec fn with_conditions(
        self,
        conditions: HorizontalPodAutoscalerConditionView,
    ) -> VHorizontalPodAutoscalerStatusView {
        VHorizontalPodAutoscalerStatusView { conditions: Some(conditions), ..self }
    }

    pub open spec fn with_current_metrics(
        self,
        current_metrics: MetricStatusView,
    ) -> VHorizontalPodAutoscalerStatusView {
        VHorizontalPodAutoscalerStatusView { current_metrics: Some(current_metrics), ..self }
    }

    pub open spec fn with_current_replicas(
        self,
        current_replicas: int,
    ) -> VHorizontalPodAutoscalerStatusView {
        VHorizontalPodAutoscalerStatusView { current_replicas: Some(current_replicas), ..self }
    }

    pub open spec fn with_desired_replicas(
        self,
        desired_replicas: int,
    ) -> VHorizontalPodAutoscalerStatusView {
        VHorizontalPodAutoscalerStatusView { desired_replicas: desired_replicas, ..self }
    }

    pub open spec fn with_last_scale_time(
        self,
        last_scale_time: TimeView,
    ) -> VHorizontalPodAutoscalerStatusView {
        VHorizontalPodAutoscalerStatusView { last_scale_time: Some(last_scale_time), ..self }
    }
}

} // verus!
