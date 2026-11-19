use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn run_suite<C: ContractInterface>(cfg: &SuiteConfig) {
    cfg.assert_validate_accepts_gen::<C>();
    cfg.assert_validate_rejects_garbage::<C>();
    cfg.assert_summarize_deterministic::<C>();
    cfg.assert_summarize_detects_structural_divergence::<C>();
    cfg.assert_delta_nonempty_and_roundtrips::<C>();
    cfg.assert_delta_handles_bad_summary::<C>();
    cfg.assert_self_delta_empty::<C>();
    cfg.assert_delta_equivalence::<C>();
    cfg.assert_update_idempotent::<C>();
    cfg.assert_update_commutative::<C>();
    cfg.assert_update_associative::<C>();
    cfg.assert_update_reads_data_not_state_plus1::<C>();
    cfg.assert_update_empty_and_unknown_noop::<C>();
    cfg.assert_update_state_and_delta::<C>();
    cfg.assert_update_rejects_garbage_data_without_panic::<C>();
}

#[cfg(test)]
mod tests {
    use crate::SuiteConfig;
    use crate::run_suite;
    use crate::testing::{DummyContract, dummy_suite_config};
    use rstest::rstest;

    #[rstest]
    #[case::validate_accepts_gen(SuiteConfig::assert_validate_accepts_gen::<DummyContract>)]
    #[case::validate_rejects_garbage(SuiteConfig::assert_validate_rejects_garbage::<DummyContract>)]
    #[case::summarize_deterministic(SuiteConfig::assert_summarize_deterministic::<DummyContract>)]
    #[case::summarize_detects_structural_divergence(
        SuiteConfig::assert_summarize_detects_structural_divergence::<DummyContract>
    )]
    #[case::delta_nonempty_and_roundtrips(
        SuiteConfig::assert_delta_nonempty_and_roundtrips::<DummyContract>
    )]
    #[case::delta_handles_bad_summary(SuiteConfig::assert_delta_handles_bad_summary::<DummyContract>)]
    #[case::update_idempotent(SuiteConfig::assert_update_idempotent::<DummyContract>)]
    #[case::update_commutative(SuiteConfig::assert_update_commutative::<DummyContract>)]
    #[case::update_associative(SuiteConfig::assert_update_associative::<DummyContract>)]
    #[case::update_reads_data_not_state_plus1(
        SuiteConfig::assert_update_reads_data_not_state_plus1::<DummyContract>
    )]
    #[case::update_empty_and_unknown_noop(
        SuiteConfig::assert_update_empty_and_unknown_noop::<DummyContract>
    )]
    #[case::update_rejects_garbage_data_without_panic(
        SuiteConfig::assert_update_rejects_garbage_data_without_panic::<DummyContract>
    )]
    #[case::self_delta_empty(SuiteConfig::assert_self_delta_empty::<DummyContract>)]
    #[case::delta_equivalence(SuiteConfig::assert_delta_equivalence::<DummyContract>)]
    #[case::state_and_delta(SuiteConfig::assert_update_state_and_delta::<DummyContract>)]
    fn contract_check(#[case] check: fn(&SuiteConfig)) {
        check(&dummy_suite_config());
    }

    #[test]
    fn test_usage() {
        run_suite::run_suite::<DummyContract>(&dummy_suite_config());
    }
}
