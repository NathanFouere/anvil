use vstd::prelude::*;

verus! {

pub enum VHorizontalPodAutoscalerReconcileStep {
    Init,
    AfterGettingScale,
    Scales(usize),
    Done,
    Error,
}

impl std::marker::Copy for VHorizontalPodAutoscalerReconcileStep {

}

impl std::clone::Clone for VHorizontalPodAutoscalerReconcileStep {
    fn clone(&self) -> (result: Self)
        ensures
            result == self,
    {
        *self
    }
}

impl View for VHorizontalPodAutoscalerReconcileStep {
    type V = VHorizontalPodAutoscalerReconcileStepView;

    open spec fn view(&self) -> VHorizontalPodAutoscalerReconcileStepView {
        match self {
            VHorizontalPodAutoscalerReconcileStep::Init => VHorizontalPodAutoscalerReconcileStepView::Init,
            VHorizontalPodAutoscalerReconcileStep::AfterGettingScale => VHorizontalPodAutoscalerReconcileStepView::AfterGettingScale,
            VHorizontalPodAutoscalerReconcileStep::Scales(
                diff,
            ) => VHorizontalPodAutoscalerReconcileStepView::Scales(*diff as nat),
            VHorizontalPodAutoscalerReconcileStep::Done => VHorizontalPodAutoscalerReconcileStepView::Done,
            VHorizontalPodAutoscalerReconcileStep::Error => VHorizontalPodAutoscalerReconcileStepView::Error,
        }
    }
}

pub enum VHorizontalPodAutoscalerReconcileStepView {
    Init,
    AfterGettingScale,
    Scales(nat),
    Done,
    Error,
}

} // verus!
