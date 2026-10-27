use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_update_reads_data_not_state_plus1<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let data = (cfg.gen_update)(&state);
    let checked_19 =
        SuiteConfig::try_apply::<C>(cfg.params.clone(), state.clone(), vec![data.clone()]);
    assert!(checked_19.is_some(), "reads_data once failed");
    let Some(once) = checked_19 else {
        return;
    };
    let checked_20 =
        SuiteConfig::try_apply::<C>(cfg.params.clone(), state, vec![data.clone(), data]);
    assert!(checked_20.is_some(), "reads_data twice failed");
    let Some(twice) = checked_20 else {
        return;
    };
    assert_eq!(
        once.as_ref(),
        twice.as_ref(),
        "update_reads_data_not_state_plus1: must be max/union from data, not +1 from state"
    );
}

// no test_usage necessary
