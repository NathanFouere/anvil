use crate::kubernetes_api_objects::spec::prelude::*;
use crate::reconciler::spec::{io::*, reconciler::*};
use crate::vhpa_controller::trusted::{spec_types::*, step::*};
use vstd::prelude::*;

verus! {

pub struct VHorizontalPodAutoscalerReconciler {}

pub struct VHorizontalPodAutoscalerReconcileState {
    pub reconcile_step: VHorizontalPodAutoscalerReconcileStepView,
}

impl Reconciler<
    VHorizontalPodAutoscalerReconcileState,
    VHorizontalPodAutoscalerView,
    VoidEReqView,
    VoidERespView,
> for VHorizontalPodAutoscalerReconciler {
    open spec fn reconcile_init_state() -> VHorizontalPodAutoscalerReconcileState {
        reconcile_init_state()
    }

    open spec fn reconcile_core(
        vrs: VHorizontalPodAutoscalerView,
        resp_o: Option<ResponseView<VoidERespView>>,
        state: VHorizontalPodAutoscalerReconcileState,
    ) -> (VHorizontalPodAutoscalerReconcileState, Option<RequestView<VoidEReqView>>) {
        reconcile_core(vrs, resp_o, state)
    }

    open spec fn reconcile_done(state: VHorizontalPodAutoscalerReconcileState) -> bool {
        reconcile_done(state)
    }

    open spec fn reconcile_error(state: VHorizontalPodAutoscalerReconcileState) -> bool {
        reconcile_error(state)
    }
}

pub open spec fn reconcile_init_state() -> VHorizontalPodAutoscalerReconcileState {
    VHorizontalPodAutoscalerReconcileState {
        reconcile_step: VHorizontalPodAutoscalerReconcileStepView::Init,
    }
}

pub open spec fn reconcile_done(state: VHorizontalPodAutoscalerReconcileState) -> bool {
    match state.reconcile_step {
        VHorizontalPodAutoscalerReconcileStepView::Done => true,
        _ => false,
    }
}

pub open spec fn reconcile_error(state: VHorizontalPodAutoscalerReconcileState) -> bool {
    match state.reconcile_step {
        VHorizontalPodAutoscalerReconcileStepView::Error => true,
        _ => false,
    }
}

// TODO
pub open spec fn reconcile_core(
    vrs: VHorizontalPodAutoscalerView,
    resp_o: Option<ResponseView<VoidERespView>>,
    state: VHorizontalPodAutoscalerReconcileState,
) -> (VHorizontalPodAutoscalerReconcileState, Option<RequestView<VoidEReqView>>) {
    let state_prime = VHorizontalPodAutoscalerReconcileState {
        reconcile_step: VHorizontalPodAutoscalerReconcileStepView::Done,
        ..state
    };
    (state_prime, None)
}

} // verus!
