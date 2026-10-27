use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_self_delta_empty<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let checked_24 = C::summarize_state(cfg.params.clone(), state.clone());
    assert!(checked_24.is_ok(), "self_delta_empty: summarize failed");
    let Ok(summary) = checked_24 else {
        return;
    };
    let checked_25 = C::get_state_delta(cfg.params.clone(), state.clone(), summary);
    assert!(
        checked_25.is_ok(),
        "self_delta_empty: get_state_delta failed"
    );
    let Ok(delta) = checked_25 else {
        return;
    };
    assert!(
        delta.as_ref().is_empty(),
        "self_delta_empty: delta against own summary must be empty (self_delta_empty #5072)"
    );
}

// no test_usage necessary
