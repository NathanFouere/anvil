use vstd::prelude::*;

verus! {

// TimeView is the ghost type of Time.
// cf . https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_37/apimachinery/pkg/apis/meta/v1/time.rs


    pub struct TimeView {
        // Abstraction choice: Time is modeled as a Unix timestamp in seconds,
        // sub-second precision is dropped.
        // TODO: to be checked, probably temporary.
        pub timestamp: int,
    }

    impl TimeView {
        pub open spec fn default() -> TimeView {
            TimeView {
                timestamp: 0,
            }
        }
    }

}
