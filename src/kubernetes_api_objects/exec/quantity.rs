use crate::kubernetes_api_objects::exec::resource::*;
use crate::kubernetes_api_objects::spec::quantity::*;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;

verus! {

// This definition is a wrapper of Quantity defined at
// https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/apimachinery/pkg/api/resource/quantity.rs
// It is supposed to be used in exec controller code.

implement_field_wrapper_type!(
    Quantity,
    k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    QuantityView
);

implement_eq!(Quantity);

impl Quantity {
    #[verifier(external_body)]
    pub fn value(&self) -> (value: String)
        ensures value@ == self@.value,
    {
        self.inner.0.clone()
    }

    #[verifier(external_body)]
    pub fn set_value(&mut self, value: String)
        ensures final(self)@ == old(self)@.with_value(value@),
    {
        self.inner.0 = value;
    }
}

}
