use freenet_stdlib::prelude::*;

use crate::basic::type_aliases::DivergentGen;
use atomic_delegate_macros::atomic_delegates;

pub struct SuiteConfig {
    pub params: Parameters<'static>,
    pub gen_state: fn() -> State<'static>,
    pub gen_update: fn(&State<'static>) -> UpdateData<'static>,
    pub gen_divergent_equal_total: Option<DivergentGen>,
    pub empty_state: fn() -> State<'static>,
}

#[atomic_delegates]
impl SuiteConfig {
    pub(crate) fn assert_validate_accepts_gen<C: ContractInterface>(&self) {}
    pub(crate) fn assert_validate_rejects_garbage<C: ContractInterface>(&self) {}
    pub(crate) fn assert_summarize_deterministic<C: ContractInterface>(&self) {}
    pub(crate) fn assert_summarize_detects_structural_divergence<C: ContractInterface>(&self) {}
    pub(crate) fn assert_delta_nonempty_and_roundtrips<C: ContractInterface>(&self) {}
    pub(crate) fn assert_delta_handles_bad_summary<C: ContractInterface>(&self) {}
    pub(crate) fn assert_update_idempotent<C: ContractInterface>(&self) {}
    pub(crate) fn assert_update_commutative<C: ContractInterface>(&self) {}
    pub(crate) fn assert_update_associative<C: ContractInterface>(&self) {}
    pub(crate) fn assert_update_reads_data_not_state_plus1<C: ContractInterface>(&self) {}
    pub(crate) fn assert_update_empty_and_unknown_noop<C: ContractInterface>(&self) {}
    pub(crate) fn assert_self_delta_empty<C: ContractInterface>(&self) {}
    pub(crate) fn assert_delta_equivalence<C: ContractInterface>(&self) {}
    pub(crate) fn assert_update_state_and_delta<C: ContractInterface>(&self) {}
    pub(crate) fn assert_update_rejects_garbage_data_without_panic<C: ContractInterface>(&self) {}
    pub(crate) fn try_apply<C: ContractInterface>(
        params: Parameters<'static>,
        state: State<'static>,
        datas: Vec<UpdateData<'static>>,
    ) -> Option<State<'static>> {
    }
}

// no test_usage necessary
