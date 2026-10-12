use freenet_stdlib::prelude::*;

use crate::basic::type_aliases::DivergentGen;

pub struct SuiteConfig {
    pub params: Parameters<'static>,
    pub gen_state: fn() -> State<'static>,
    pub gen_update: fn(&State<'static>) -> UpdateData<'static>,
    pub gen_divergent_equal_total: Option<DivergentGen>,
    pub empty_state: fn() -> State<'static>,
}
