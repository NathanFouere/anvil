#![allow(unused_imports)]

use crate::kubernetes_api_objects::exec::prelude::*;
use crate::kubernetes_api_objects::spec::prelude::*;
use crate::reconciler::exec::{io::*, reconciler::*};
use crate::memcached_controller::model::reconciler as model_reconciler;
use crate::memcached_controller::trusted::{exec_types::*, step::*};
use crate::vstd_ext::{string_map::*, string_view::*};
use vstd::prelude::*;
use crate::reconciler::spec::io::*;

verus! {

pub struct MemcachedReconcileState {
    pub reconcile_step: MemcachedReconcileStep,
}

impl View for MemcachedReconcileState {
    type V = model_reconciler::MemcachedReconcileState;

    open spec fn view(&self) -> model_reconciler::MemcachedReconcileState {
        model_reconciler::MemcachedReconcileState {
            reconcile_step: self.reconcile_step@,
        }
    }
}

pub struct MemcachedReconciler {}

impl Reconciler for MemcachedReconciler {
    type S = MemcachedReconcileState;
    type K = MemcachedCluster;
    type EReq = VoidEReq;
    type EResp = VoidEResp;
    type M = model_reconciler::MemcachedReconciler;

    fn reconcile_init_state() -> Self::S {
        reconcile_init_state()
    }

    fn reconcile_core(memcached: &Self::K, resp_o: Option<Response<Self::EResp>>, state: Self::S) -> (Self::S, Option<Request<Self::EReq>>) {
        reconcile_core(memcached, resp_o, state)
    }

    fn reconcile_done(state: &Self::S) -> bool {
        reconcile_done(state)
    }

    fn reconcile_error(state: &Self::S) -> bool {
        reconcile_error(state)
    }
}

pub fn reconcile_init_state() -> (state: MemcachedReconcileState)
    ensures state@ == model_reconciler::reconcile_init_state(),
{
    MemcachedReconcileState {
        reconcile_step: MemcachedReconcileStep::Init,
    }
}

pub fn reconcile_done(state: &MemcachedReconcileState) -> (res: bool)
    ensures res == model_reconciler::reconcile_done(state@),
{
    match state.reconcile_step {
        MemcachedReconcileStep::Done => true,
        _ => false,
    }
}

pub fn reconcile_error(state: &MemcachedReconcileState) -> (res: bool)
    ensures res == model_reconciler::reconcile_error(state@),
{
    match state.reconcile_step {
        MemcachedReconcileStep::Error => true,
        _ => false,
    }
}

pub fn reconcile_core(memcached: &MemcachedCluster, resp_o: Option<Response<VoidEResp>>, state: MemcachedReconcileState) -> (res: (MemcachedReconcileState, Option<Request<VoidEReq>>))
    requires memcached@.well_formed(),
    ensures (res.0@, res.1.deep_view()) == model_reconciler::reconcile_core(memcached@, resp_o.deep_view(), state@),
{
    match state.reconcile_step {
        MemcachedReconcileStep::Init => {
            (error_state(state), None)
        },
        MemcachedReconcileStep::AfterGetDeployment => {
            (error_state(state), None)
        },
        MemcachedReconcileStep::AfterCreateDeployment => {
            (error_state(state), None)
        },
        MemcachedReconcileStep::AfterScaleDeployment => {
            (error_state(state), None)
        },
        MemcachedReconcileStep::AfterInitStatus => {
            (error_state(state), None)
        },
        MemcachedReconcileStep::AfterSetAvailableStates => {
            (error_state(state), None)
        },
        _ => {
            (state, None)
        },
    }
}

pub fn error_state(state: MemcachedReconcileState) -> (state_prime: MemcachedReconcileState)
    ensures state_prime@ == model_reconciler::error_state(state@),
{
    MemcachedReconcileState {
        reconcile_step: MemcachedReconcileStep::Error,
        ..state
    }
}

pub fn done_state(state: MemcachedReconcileState) -> (state_prime: MemcachedReconcileState)
    ensures state_prime@ == model_reconciler::done_state(state@),
{
    MemcachedReconcileState {
        reconcile_step: MemcachedReconcileStep::Done,
        ..state
    }
}

}
