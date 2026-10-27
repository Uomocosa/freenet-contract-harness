use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_delta_handles_bad_summary<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let bad = StateSummary::from(b"garbage".to_vec());
    let checked_10 = C::get_state_delta(cfg.params.clone(), state.clone(), bad);
    assert!(
        checked_10.is_ok(),
        "get_state_delta with bad summary must not error"
    );
    let Ok(delta) = checked_10 else {
        return;
    };
    let empty = (cfg.empty_state)();
    let result = C::update_state(cfg.params.clone(), empty, vec![UpdateData::Delta(delta)]);
    assert!(
        result.is_ok(),
        "delta_handles_bad_summary: must fallback to whole-state, not panic, got {result:?}"
    );
}

// no test_usage necessary
