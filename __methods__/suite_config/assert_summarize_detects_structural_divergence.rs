use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_summarize_detects_structural_divergence<C: ContractInterface>(cfg: &SuiteConfig) {
    let Some(divergent) = cfg.gen_divergent_equal_total else {
        return;
    };
    let Some((a, b)) = divergent() else {
        return;
    };
    let checked_3 = C::summarize_state(cfg.params.clone(), a);
    assert!(checked_3.is_ok(), "summarize divergent a failed");
    let Ok(sa) = checked_3 else {
        return;
    };
    let checked_4 = C::summarize_state(cfg.params.clone(), b);
    assert!(checked_4.is_ok(), "summarize divergent b failed");
    let Ok(sb) = checked_4 else {
        return;
    };
    assert_ne!(
        sa.as_ref(),
        sb.as_ref(),
        "summarize_detects_structural_divergence: equal totals with different shape must have different summaries (per-tag summary required)"
    );
}

// no test_usage necessary
