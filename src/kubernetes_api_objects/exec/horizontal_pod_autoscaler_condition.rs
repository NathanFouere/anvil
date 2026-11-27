use crate::kubernetes_api_objects::exec::{resource::*, time::*};
use crate::kubernetes_api_objects::spec::horizontal_pod_autoscaler_condition::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of HorizontalPodAutoscalerCondition defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/horizontal_pod_autoscaler_condition.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    HorizontalPodAutoscalerCondition,
    k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerCondition,
    HorizontalPodAutoscalerConditionView
);

impl HorizontalPodAutoscalerCondition {
    #[verifier(external_body)]
    pub fn last_transition_time(&self) -> (last_transition_time: Option<Time>)
        ensures self@.last_transition_time == last_transition_time.deep_view(),
    {
        match &self.inner.last_transition_time {
            Some(t) => Some(Time::from_kube(t.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn message(&self) -> (message: Option<String>)
        ensures self@.message == opt_string_to_view(&message),
    {
        self.inner.message.clone()
    }

    #[verifier(external_body)]
    pub fn reason(&self) -> (reason: Option<String>)
        ensures self@.reason == opt_string_to_view(&reason),
    {
        self.inner.reason.clone()
    }

    #[verifier(external_body)]
    pub fn status(&self) -> (status: String)
        ensures status@ == self@.status,
    {
        self.inner.status.clone()
    }

    #[verifier(external_body)]
    pub fn type_(&self) -> (type_: String)
        ensures type_@ == self@.type_,
    {
        self.inner.type_.clone()
    }
}

}
