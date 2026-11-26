use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// QuantityView is the ghost type of Quantity.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/apimachinery/pkg/api/resource/quantity.rs

pub struct QuantityView {
    pub value: StringView,
}

impl QuantityView {
    pub open spec fn default() -> QuantityView {
        QuantityView {
            value: ""@,
        }
    }

    pub open spec fn with_value(self, value: StringView) -> QuantityView {
        QuantityView {
            value: value,
            ..self
        }
    }
}

}
