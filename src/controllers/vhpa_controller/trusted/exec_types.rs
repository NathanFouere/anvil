// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::error::UnmarshalError;
use crate::kubernetes_api_objects::exec::{
    api_resource::*, horizontal_pod_autoscaler_behavior::*, horizontal_pod_autoscaler_condition::*,
    metric_spec::*, metric_status::*, prelude::*, time::*,
};
use crate::kubernetes_api_objects::spec::resource::*;
use crate::vhpa_controller::trusted::spec_types;
use kube::Resource;
use vstd::prelude::*;

verus! {

implement_object_wrapper_type!(
    VHorizontalPodAutoscaler,
    crate::crds::VHorizontalPodAutoscaler,
    spec_types::VHorizontalPodAutoscalerView
);

implement_field_wrapper_type!(
    VHorizontalPodAutoscalerSpec,
    crate::crds::VHorizontalPodAutoscalerSpec,
    spec_types::VHorizontalPodAutoscalerSpecView
);

implement_field_wrapper_type!(
    VHorizontalPodAutoscalerStatus,
    crate::crds::VHorizontalPodAutoscalerStatus,
    spec_types::VHorizontalPodAutoscalerStatusView
);

impl VHorizontalPodAutoscaler {
    #[verifier(external_body)]
    pub fn well_formed(&self) -> (b: bool)
        ensures
            b == self@.well_formed(),
    {
        self.metadata().well_formed_for_namespaced() && self.state_validation()
    }

    #[verifier(external_body)]
    pub fn spec(&self) -> (spec: VHorizontalPodAutoscalerSpec)
        ensures
            spec@ == self@.spec,
    {
        VHorizontalPodAutoscalerSpec { inner: self.inner.spec.clone() }
    }

    #[verifier(external_body)]
    pub fn status(&self) -> (status: Option<VHorizontalPodAutoscalerStatus>)
        ensures
            status is Some == self@.status is Some,
            status is Some ==> status->0@ == self@.status->0,
    {
        match &self.inner.status {
            Some(s) => Some(VHorizontalPodAutoscalerStatus { inner: s.clone() }),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn controller_owner_ref(&self) -> (owner_reference: OwnerReference)
        ensures
            owner_reference@ == self@.controller_owner_ref(),
    {
        // We can safely unwrap here because the trait method implementation always returns a Some(...)
        OwnerReference::from_kube(self.inner.controller_owner_ref(&()).unwrap())
    }

    #[verifier(external_body)]
    pub fn set_spec(&mut self, spec: VHorizontalPodAutoscalerSpec)
        ensures
            final(self)@ == old(self)@.with_spec(spec@),
    {
        self.inner.spec = spec.into_kube();
    }

    #[verifier(external_body)]
    pub fn set_status(&mut self, status: VHorizontalPodAutoscalerStatus)
        ensures
            final(self)@ == old(self)@.with_status(status@),
    {
        self.inner.status = Some(status.into_kube());
    }

    // TODO: keep it consistent with spec_types::VHorizontalPodAutoscalerView::_state_validation
    pub fn state_validation(&self) -> (res: bool)
        ensures
            res == self@.state_validation(),
    {
        true
    }
}

impl VHorizontalPodAutoscalerSpec {
    #[verifier(external_body)]
    pub fn behavior(&self) -> (behavior: Option<HorizontalPodAutoscalerBehavior>)
        ensures
            behavior is Some == self@.behavior is Some,
            behavior is Some ==> behavior->0@ == self@.behavior->0,
    {
        match &self.inner.behavior {
            Some(b) => Some(HorizontalPodAutoscalerBehavior::from_kube(b.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn max_replicas(&self) -> (max_replicas: i32)
        ensures
            max_replicas as int == self@.max_replicas,
    {
        self.inner.max_replicas
    }

    #[verifier(external_body)]
    pub fn metrics(&self) -> (metrics: Option<MetricSpec>)
        ensures
            metrics is Some == self@.metrics is Some,
            metrics is Some ==> metrics->0@ == self@.metrics->0,
    {
        match &self.inner.metrics {
            Some(m) => Some(MetricSpec::from_kube(m.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn min_replicas(&self) -> (min_replicas: Option<i32>)
        ensures
            min_replicas is Some == self@.min_replicas is Some,
            min_replicas is Some ==> min_replicas->0 as int == self@.min_replicas->0,
    {
        self.inner.min_replicas
    }

    #[verifier(external_body)]
    pub fn set_behavior(&mut self, behavior: HorizontalPodAutoscalerBehavior)
        ensures
            final(self)@ == old(self)@.with_behavior(behavior@),
    {
        self.inner.behavior = Some(behavior.into_kube());
    }

    #[verifier(external_body)]
    pub fn set_max_replicas(&mut self, max_replicas: i32)
        ensures
            final(self)@ == old(self)@.with_max_replicas(max_replicas as int),
    {
        self.inner.max_replicas = max_replicas;
    }

    #[verifier(external_body)]
    pub fn set_metrics(&mut self, metrics: MetricSpec)
        ensures
            final(self)@ == old(self)@.with_metrics(metrics@),
    {
        self.inner.metrics = Some(metrics.into_kube());
    }

    #[verifier(external_body)]
    pub fn set_min_replicas(&mut self, min_replicas: i32)
        ensures
            final(self)@ == old(self)@.with_min_replicas(min_replicas as int),
    {
        self.inner.min_replicas = Some(min_replicas);
    }
}

impl VHorizontalPodAutoscalerStatus {
    #[verifier(external_body)]
    pub fn conditions(&self) -> (conditions: Option<HorizontalPodAutoscalerCondition>)
        ensures
            conditions is Some == self@.conditions is Some,
            conditions is Some ==> conditions->0@ == self@.conditions->0,
    {
        match &self.inner.conditions {
            Some(c) => Some(HorizontalPodAutoscalerCondition::from_kube(c.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn current_metrics(&self) -> (current_metrics: Option<MetricStatus>)
        ensures
            current_metrics is Some == self@.current_metrics is Some,
            current_metrics is Some ==> current_metrics->0@ == self@.current_metrics->0,
    {
        match &self.inner.current_metrics {
            Some(m) => Some(MetricStatus::from_kube(m.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn current_replicas(&self) -> (current_replicas: Option<i32>)
        ensures
            current_replicas is Some == self@.current_replicas is Some,
            current_replicas is Some ==> current_replicas->0 as int == self@.current_replicas->0,
    {
        self.inner.current_replicas
    }

    #[verifier(external_body)]
    pub fn desired_replicas(&self) -> (desired_replicas: i32)
        ensures
            desired_replicas as int == self@.desired_replicas,
    {
        self.inner.desired_replicas
    }

    #[verifier(external_body)]
    pub fn last_scale_time(&self) -> (last_scale_time: Option<Time>)
        ensures
            last_scale_time is Some == self@.last_scale_time is Some,
            last_scale_time is Some ==> last_scale_time->0@ == self@.last_scale_time->0,
    {
        match &self.inner.last_scale_time {
            Some(t) => Some(Time::from_kube(t.clone())),
            None => None,
        }
    }

    #[verifier(external_body)]
    pub fn set_conditions(&mut self, conditions: HorizontalPodAutoscalerCondition)
        ensures
            final(self)@ == old(self)@.with_conditions(conditions@),
    {
        self.inner.conditions = Some(conditions.into_kube());
    }

    #[verifier(external_body)]
    pub fn set_current_metrics(&mut self, current_metrics: MetricStatus)
        ensures
            final(self)@ == old(self)@.with_current_metrics(current_metrics@),
    {
        self.inner.current_metrics = Some(current_metrics.into_kube());
    }

    #[verifier(external_body)]
    pub fn set_current_replicas(&mut self, current_replicas: i32)
        ensures
            final(self)@ == old(self)@.with_current_replicas(current_replicas as int),
    {
        self.inner.current_replicas = Some(current_replicas);
    }

    #[verifier(external_body)]
    pub fn set_desired_replicas(&mut self, desired_replicas: i32)
        ensures
            final(self)@ == old(self)@.with_desired_replicas(desired_replicas as int),
    {
        self.inner.desired_replicas = desired_replicas;
    }

    #[verifier(external_body)]
    pub fn set_last_scale_time(&mut self, last_scale_time: Time)
        ensures
            final(self)@ == old(self)@.with_last_scale_time(last_scale_time@),
    {
        self.inner.last_scale_time = Some(last_scale_time.into_kube());
    }
}

} // verus!
