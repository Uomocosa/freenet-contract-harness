use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_update_idempotent<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let data = (cfg.gen_update)(&state);
    let checked_11 =
        SuiteConfig::try_apply::<C>(cfg.params.clone(), state.clone(), vec![data.clone()]);
    assert!(checked_11.is_some(), "update_idempotent once failed");
    let Some(once) = checked_11 else {
        return;
    };
    let checked_12 = SuiteConfig::try_apply::<C>(
        cfg.params.clone(),
        state.clone(),
        vec![data.clone(), data.clone()],
    );
    assert!(checked_12.is_some(), "update_idempotent twice failed");
    let Some(twice) = checked_12 else {
        return;
    };
    assert_eq!(
        once.as_ref(),
        twice.as_ref(),
        "update_idempotent: applying same update twice must equal once"
    );
    let checked_13 = SuiteConfig::try_apply::<C>(cfg.params.clone(), once.clone(), vec![data]);
    assert!(checked_13.is_some(), "reapply failed");
    let Some(reapply) = checked_13 else {
        return;
    };
    assert_eq!(
        once.as_ref(),
        reapply.as_ref(),
        "update_idempotent: reapplying after merge must be no-op"
    );
}

// no test_usage necessary
