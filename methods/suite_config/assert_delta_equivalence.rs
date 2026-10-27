use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_delta_equivalence<C: ContractInterface>(cfg: &SuiteConfig) {
    let base = (cfg.gen_state)();
    let update = (cfg.gen_update)(&base);
    let checked_26 = SuiteConfig::try_apply::<C>(cfg.params.clone(), base.clone(), vec![update]);
    assert!(
        checked_26.is_some(),
        "delta_equivalence: ahead apply failed"
    );
    let Some(ahead) = checked_26 else {
        return;
    };
    if ahead.as_ref() == base.as_ref() {
        return;
    }
    let checked_27 = C::summarize_state(cfg.params.clone(), base.clone());
    assert!(
        checked_27.is_ok(),
        "delta_equivalence: summarize base failed"
    );
    let Ok(summary) = checked_27 else {
        return;
    };
    let checked_28 = C::get_state_delta(cfg.params.clone(), ahead.clone(), summary);
    assert!(
        checked_28.is_ok(),
        "delta_equivalence: get_state_delta failed"
    );
    let Ok(delta) = checked_28 else {
        return;
    };
    assert!(
        !delta.as_ref().is_empty(),
        "delta_equivalence: delta should be non-empty"
    );
    let checked_29 = SuiteConfig::try_apply::<C>(
        cfg.params.clone(),
        base.clone(),
        vec![UpdateData::Delta(delta)],
    );
    assert!(
        checked_29.is_some(),
        "delta_equivalence: via_delta apply failed"
    );
    let Some(via_delta) = checked_29 else {
        return;
    };
    let checked_35 = SuiteConfig::try_apply::<C>(
        cfg.params.clone(),
        base,
        vec![UpdateData::State(ahead.clone())],
    );
    assert!(
        checked_35.is_some(),
        "delta_equivalence: via_state apply failed"
    );
    let Some(via_state) = checked_35 else {
        return;
    };
    assert_eq!(
        via_delta.as_ref(),
        via_state.as_ref(),
        "delta_equivalence: delta and whole-state merge must be equivalent (R ∪ (S\\R) == R ∪ S)"
    );
    assert_eq!(
        via_delta.as_ref(),
        ahead.as_ref(),
        "delta_equivalence: result must equal ahead"
    );
}

// no test_usage necessary
