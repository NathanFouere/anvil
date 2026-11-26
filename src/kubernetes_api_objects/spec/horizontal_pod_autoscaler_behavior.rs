// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::spec::hpa_scaling_rules::HPAScalingRulesView;
use vstd::prelude::*;

verus! {

// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/horizontal_pod_autoscaler_behavior.rs
// HorizontalPodAutoscalerBehaviorView is the ghost type of HorizontalPodAutoscalerBehavior.

pub struct HorizontalPodAutoscalerBehaviorView {
    pub scale_down: Option<HPAScalingRulesView>,
    pub scale_up: Option<HPAScalingRulesView>,
}

impl HorizontalPodAutoscalerBehaviorView {
    pub open spec fn default() -> HorizontalPodAutoscalerBehaviorView {
        HorizontalPodAutoscalerBehaviorView {
            scale_down: None,
            scale_up: None,
        }
    }

    pub open spec fn with_scale_down(self, scale_down: HPAScalingRulesView) -> HorizontalPodAutoscalerBehaviorView {
        HorizontalPodAutoscalerBehaviorView {
            scale_down: Some(scale_down),
            ..self
        }
    }

    pub open spec fn with_scale_up(self, scale_up: HPAScalingRulesView) -> HorizontalPodAutoscalerBehaviorView {
        HorizontalPodAutoscalerBehaviorView {
            scale_up: Some(scale_up),
            ..self
        }
    }
}

}
