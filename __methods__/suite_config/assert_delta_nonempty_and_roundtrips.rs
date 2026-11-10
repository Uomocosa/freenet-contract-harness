use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_delta_nonempty_and_roundtrips<C: ContractInterface>(cfg: &SuiteConfig) {
    let base = (cfg.gen_state)();
    let update = (cfg.gen_update)(&base);
    let checked_5 = SuiteConfig::try_apply::<C>(cfg.params.clone(), base.clone(), vec![update]);
    assert!(
        checked_5.is_some(),
        "delta_nonempty_and_roundtrips: apply ahead failed"
    );
    let Some(ahead) = checked_5 else {
        return;
    };
    if ahead.as_ref() == base.as_ref() {
        return;
    }
    let checked_6 = C::summarize_state(cfg.params.clone(), base.clone());
    assert!(checked_6.is_ok(), "summarize base for delta failed");
    let Ok(summary) = checked_6 else {
        return;
    };
    let checked_7 = C::get_state_delta(cfg.params.clone(), ahead.clone(), summary);
    assert!(checked_7.is_ok(), "get_state_delta failed");
    let Ok(delta) = checked_7 else {
        return;
    };
    assert!(
        !delta.as_ref().is_empty(),
        "delta_nonempty_and_roundtrips: empty delta disables anti-entropy"
    );
    let checked_8 = C::update_state(cfg.params.clone(), base, vec![UpdateData::Delta(delta)]);
    assert!(checked_8.is_ok(), "delta update failed");
    let Ok(merged) = checked_8 else {
        return;
    };
    let checked_9 = merged.new_state;
    assert!(checked_9.is_some(), "delta merge returned no state");
    let Some(merged_state) = checked_9 else {
        return;
    };
    assert_eq!(
        merged_state.as_ref(),
        ahead.as_ref(),
        "delta_nonempty_and_roundtrips: delta must converge peer"
    );
}

// no test_usage necessary
