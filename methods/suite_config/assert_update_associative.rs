use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_update_associative<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let a = (cfg.gen_update)(&state);
    let b = (cfg.gen_update)(&state);
    let c = (cfg.gen_update)(&state);
    let checked_16 = SuiteConfig::try_apply::<C>(
        cfg.params.clone(),
        state.clone(),
        vec![a.clone(), b.clone()],
    );
    assert!(checked_16.is_some(), "associative ab failed");
    let Some(ab) = checked_16 else {
        return;
    };
    let checked_17 = SuiteConfig::try_apply::<C>(cfg.params.clone(), ab, vec![c.clone()]);
    assert!(checked_17.is_some(), "associative ab_c failed");
    let Some(ab_c) = checked_17 else {
        return;
    };
    let checked_18 = SuiteConfig::try_apply::<C>(cfg.params.clone(), state, vec![a, b, c]);
    assert!(checked_18.is_some(), "associative a_bc failed");
    let Some(a_bc) = checked_18 else {
        return;
    };
    assert_eq!(
        ab_c.as_ref(),
        a_bc.as_ref(),
        "update_associative: batch vs split apply must be equal"
    );
}

// no test_usage necessary
