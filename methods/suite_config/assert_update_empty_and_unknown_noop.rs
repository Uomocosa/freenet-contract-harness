use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_update_empty_and_unknown_noop<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let checked_21 = SuiteConfig::try_apply::<C>(cfg.params.clone(), state.clone(), vec![]);
    assert!(checked_21.is_some(), "empty apply failed");
    let Some(empty) = checked_21 else {
        return;
    };
    assert_eq!(
        empty.as_ref(),
        state.as_ref(),
        "update_empty_and_unknown_noop: empty data must be no-op"
    );
    let related_to = ContractInstanceId::new([9u8; 32]);
    let checked_22 = C::update_state(
        cfg.params.clone(),
        state.clone(),
        vec![UpdateData::RelatedState {
            related_to,
            state: State::from(Vec::new()),
        }],
    );
    assert!(checked_22.is_ok(), "Related variant must not error");
    let Ok(with_related) = checked_22 else {
        return;
    };
    let checked_23 = with_related.new_state;
    assert!(checked_23.is_some(), "Related returned no state");
    let Some(related_state) = checked_23 else {
        return;
    };
    assert_eq!(
        related_state.as_ref(),
        state.as_ref(),
        "update_empty_and_unknown_noop: Related variant must be no-op"
    );
}

// no test_usage necessary
