use vstd::prelude::*;

verus! {

pub enum MemcachedReconcileStep {
    Init,
    AfterInitStatus,
    AfterGetDeployment,
    AfterCreateDeployment,
    AfterScaleDeployment,
    AfterSetAvailableStates,
    Done,
    Error,
}

impl std::marker::Copy for MemcachedReconcileStep {}

impl std::clone::Clone for MemcachedReconcileStep {
    fn clone(&self) -> (result: Self)
        ensures result == self
    { *self }
}

impl View for MemcachedReconcileStep {
    type V = MemcachedReconcileStepView;

    open spec fn view(&self) -> MemcachedReconcileStepView {
        match self {
            MemcachedReconcileStep::Init => MemcachedReconcileStepView::Init,
            MemcachedReconcileStep::AfterInitStatus => MemcachedReconcileStepView::AfterInitStatus,
            MemcachedReconcileStep::AfterGetDeployment => MemcachedReconcileStepView::AfterGetDeployment,
            MemcachedReconcileStep::AfterCreateDeployment => MemcachedReconcileStepView::AfterCreateDeployment,
            MemcachedReconcileStep::AfterScaleDeployment => MemcachedReconcileStepView::AfterScaleDeployment,
            MemcachedReconcileStep::AfterSetAvailableStates => MemcachedReconcileStepView::AfterSetAvailableStates,
            MemcachedReconcileStep::Done => MemcachedReconcileStepView::Done,
            MemcachedReconcileStep::Error => MemcachedReconcileStepView::Error,
        }
    }f
}

pub enum MemcachedReconcileStepView {
    Init,
    AfterInitStatus,
    AfterGetDeployment,
    AfterCreateDeployment,
    AfterScaleDeployment,
    AfterSetAvailableStates,
    Done,
    Error,
}

}
