#![allow(unused_imports)]

use crate::kubernetes_api_objects::spec::prelude::*;
use crate::reconciler::spec::{io::*, reconciler::*};
use crate::memcached_controller::trusted::{spec_types::*, step::*};
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

pub struct MemcachedReconciler {}

pub struct MemcachedReconcileState {
    pub reconcile_step: MemcachedReconcileStepView,
}

impl Reconciler<MemcachedReconcileState, MemcachedClusterView, VoidEReqView, VoidERespView> for MemcachedReconciler {
    open spec fn reconcile_init_state() -> MemcachedReconcileState {
        reconcile_init_state()
    }

    open spec fn reconcile_core(memcached: MemcachedClusterView, resp_o: Option<ResponseView<VoidERespView>>, state: MemcachedReconcileState) -> (MemcachedReconcileState, Option<RequestView<VoidEReqView>>) {
        reconcile_core(memcached, resp_o, state)
    }

    open spec fn reconcile_done(state: MemcachedReconcileState) -> bool {
        reconcile_done(state)
    }

    open spec fn reconcile_error(state: MemcachedReconcileState) -> bool {
        reconcile_error(state)
    }
}

pub open spec fn reconcile_init_state() -> MemcachedReconcileState {
    MemcachedReconcileState {
        reconcile_step: MemcachedReconcileStepView::Init,
    }
}

pub open spec fn reconcile_done(state: MemcachedReconcileState) -> bool {
    match state.reconcile_step {
        MemcachedReconcileStepView::Done => true,
        _ => false,
    }
}

pub open spec fn reconcile_error(state: MemcachedReconcileState) -> bool {
    match state.reconcile_step {
        MemcachedReconcileStepView::Error => true,
        _ => false,
    }
}

pub open spec fn reconcile_core(memcached: MemcachedClusterView, resp_o: Option<ResponseView<VoidERespView>>, state: MemcachedReconcileState) -> (res: (MemcachedReconcileState, Option<RequestView<VoidEReqView>>)) {
    match state.reconcile_step {
        MemcachedReconcileStepView::Init => {
            (error_state(state), None)
        },
        MemcachedReconcileStepView::AfterGetDeployment => {
            (error_state(state), None)
        },
        MemcachedReconcileStepView::AfterCreateDeployment => {
            (error_state(state), None)
        },
        MemcachedReconcileStepView::AfterScaleDeployment => {
            (error_state(state), None)
        },
        MemcachedReconcileStepView::AfterInitStatus => {
            (error_state(state), None)
        },
        MemcachedReconcileStepView::AfterSetAvailableStates => {
            (error_state(state), None)
        },
        _ => {
            (state, None)
        },
    }
}

pub open spec fn error_state(state: MemcachedReconcileState) -> (state_prime: MemcachedReconcileState) {
    MemcachedReconcileState {
        reconcile_step: MemcachedReconcileStepView::Error,
        ..state
    }
}

pub open spec fn done_state(state: MemcachedReconcileState) -> (state_prime: MemcachedReconcileState) {
    MemcachedReconcileState {
        reconcile_step: MemcachedReconcileStepView::Done,
        ..state
    }
}

}
