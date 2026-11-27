use crate::kubernetes_api_objects::exec::prelude::*;
use crate::kubernetes_api_objects::spec::prelude::*;
use crate::reconciler::exec::{io::*, reconciler::*};
use crate::reconciler::spec::io::*;
use crate::vhpa_controller::model::reconciler as model_reconciler;
use crate::vhpa_controller::trusted::{
    exec_types::*, spec_types::VHorizontalPodAutoscalerView, step::*,
};
use crate::vstd_ext::{seq_lib::*, string_map::StringMap, string_view::*, vec_lib::*};
use vstd::prelude::*;
use vstd::seq_lib::*;

verus! {

pub struct VHorizontalPodAutoscalerReconcileState {
    pub reconcile_step: VHorizontalPodAutoscalerReconcileStep,
}

impl View for VHorizontalPodAutoscalerReconcileState {
    type V = model_reconciler::VHorizontalPodAutoscalerReconcileState;

    open spec fn view(&self) -> model_reconciler::VHorizontalPodAutoscalerReconcileState {
        model_reconciler::VHorizontalPodAutoscalerReconcileState {
            reconcile_step: self.reconcile_step@,
        }
    }
}

pub struct VHorizontalPodAutoscalerReconciler {}

impl Reconciler for VHorizontalPodAutoscalerReconciler {
    type S = VHorizontalPodAutoscalerReconcileState;

    type K = VHorizontalPodAutoscaler;

    type EReq = VoidEReq;

    type EResp = VoidEResp;

    type M = model_reconciler::VHorizontalPodAutoscalerReconciler;

    fn reconcile_init_state() -> Self::S {
        reconcile_init_state()
    }

    fn reconcile_core(vrs: &Self::K, resp_o: Option<Response<Self::EResp>>, state: Self::S) -> (
        Self::S,
        Option<Request<Self::EReq>>,
    ) {
        reconcile_core(vrs, resp_o, state)
    }

    fn reconcile_done(state: &Self::S) -> bool {
        reconcile_done(state)
    }

    fn reconcile_error(state: &Self::S) -> bool {
        reconcile_error(state)
    }
}

pub fn reconcile_init_state() -> (state: VHorizontalPodAutoscalerReconcileState)
    ensures
        state@ == model_reconciler::reconcile_init_state(),
{
    VHorizontalPodAutoscalerReconcileState {
        reconcile_step: VHorizontalPodAutoscalerReconcileStep::Init,
    }
}

pub fn reconcile_done(state: &VHorizontalPodAutoscalerReconcileState) -> (res: bool)
    ensures
        res == model_reconciler::reconcile_done(state@),
{
    match state.reconcile_step {
        VHorizontalPodAutoscalerReconcileStep::Done => true,
        _ => false,
    }
}

pub fn reconcile_error(state: &VHorizontalPodAutoscalerReconcileState) -> (res: bool)
    ensures
        res == model_reconciler::reconcile_error(state@),
{
    match state.reconcile_step {
        VHorizontalPodAutoscalerReconcileStep::Error => true,
        _ => false,
    }
}

pub fn reconcile_core(
    vd: &VHorizontalPodAutoscaler,
    resp_o: Option<Response<VoidEResp>>,
    state: VHorizontalPodAutoscalerReconcileState,
) -> (res: (VHorizontalPodAutoscalerReconcileState, Option<Request<VoidEReq>>))
    requires
        vd@.well_formed(),
    ensures
        (res.0@, res.1.deep_view()) == model_reconciler::reconcile_core(
            vd@,
            resp_o.deep_view(),
            state@,
        ),
{
    let state_prime = VHorizontalPodAutoscalerReconcileState {
        reconcile_step: VHorizontalPodAutoscalerReconcileStep::Done,
    };
    return (state_prime, None)
}

} // verus!
