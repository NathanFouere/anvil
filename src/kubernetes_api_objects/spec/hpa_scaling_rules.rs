use crate::{kubernetes_api_objects::spec::hpa_scaling_policy::HPAScalingPolicyView, vstd_ext::string_view::*};
use vstd::prelude::*;

verus! {

// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/api/autoscaling/v2/hpa_scaling_rules.rs
// HPAScalingRulesView is the ghost type of HPAScalingRules.

pub struct HPAScalingRulesView {
    pub policies: Option<Seq<HPAScalingPolicyView>>,
    pub select_policy: Option<StringView>,
    pub stabilization_window_seconds: Option<int>,
    // TODO = pour le mont on ajoute pas le champs "Tolerance", à voir si on le metà l'avenir
}

impl HPAScalingRulesView {
    pub open spec fn default() -> HPAScalingRulesView {
        HPAScalingRulesView {
            policies: None,
            select_policy: None,
            stabilization_window_seconds: None,
        }
    }

    pub open spec fn with_policies(self, policies: Seq<HPAScalingPolicyView>) -> HPAScalingRulesView {
        HPAScalingRulesView {
            policies: Some(policies),
            ..self
        }
    }

    pub open spec fn with_select_policy(self, select_policy: StringView) -> HPAScalingRulesView {
        HPAScalingRulesView {
            select_policy: Some(select_policy),
            ..self
        }
    }

    pub open spec fn with_stabilization_window_seconds(self, stabilization_window_seconds: int) -> HPAScalingRulesView {
        HPAScalingRulesView {
            stabilization_window_seconds: Some(stabilization_window_seconds),
            ..self
        }
    }
}

}
