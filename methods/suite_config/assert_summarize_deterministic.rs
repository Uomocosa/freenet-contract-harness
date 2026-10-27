use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_summarize_deterministic<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let checked_1 = C::summarize_state(cfg.params.clone(), state.clone());
    assert!(
        checked_1.is_ok(),
        "summarize_deterministic: first summarize failed"
    );
    let Ok(a) = checked_1 else {
        return;
    };
    let checked_2 = C::summarize_state(cfg.params.clone(), state);
    assert!(
        checked_2.is_ok(),
        "summarize_deterministic: second summarize failed"
    );
    let Ok(b) = checked_2 else {
        return;
    };
    assert_eq!(
        a.as_ref(),
        b.as_ref(),
        "summarize_deterministic: same state must give same summary"
    );
}

// no test_usage necessary
