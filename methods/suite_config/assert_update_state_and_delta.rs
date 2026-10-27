use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_update_state_and_delta<C: ContractInterface>(cfg: &SuiteConfig) {
    let base = (cfg.gen_state)();
    let update_a = (cfg.gen_update)(&base);
    let checked_30 =
        SuiteConfig::try_apply::<C>(cfg.params.clone(), base.clone(), vec![update_a.clone()]);
    assert!(
        checked_30.is_some(),
        "state_and_delta: first update apply failed"
    );
    let Some(ahead_a) = checked_30 else {
        return;
    };
    if ahead_a.as_ref() == base.as_ref() {
        return;
    }
    let delta_bytes = match &update_a {
        UpdateData::State(s) => s.as_ref().to_vec(),
        UpdateData::Delta(d) => d.as_ref().to_vec(),
        _ => return,
    };
    let state_bytes = ahead_a.as_ref().to_vec();
    let checked_31 = C::update_state(
        cfg.params.clone(),
        base.clone(),
        vec![UpdateData::StateAndDelta {
            state: State::from(Vec::new()),
            delta: StateDelta::from(Vec::new()),
        }],
    );
    assert!(
        checked_31.is_ok(),
        "state_and_delta: empty StateAndDelta must not error"
    );
    let Ok(empty_apply) = checked_31 else {
        return;
    };
    let checked_32 = empty_apply.new_state;
    assert!(
        checked_32.is_some(),
        "state_and_delta: empty returned no state"
    );
    let Some(empty_state) = checked_32 else {
        return;
    };
    assert_eq!(
        empty_state.as_ref(),
        base.as_ref(),
        "state_and_delta: empty StateAndDelta must be no-op"
    );
    let checked_33 = C::update_state(
        cfg.params.clone(),
        base.clone(),
        vec![UpdateData::StateAndDelta {
            state: State::from(state_bytes),
            delta: StateDelta::from(delta_bytes),
        }],
    );
    assert!(checked_33.is_ok(), "state_and_delta: combined apply failed");
    let Ok(both) = checked_33 else {
        return;
    };
    let checked_34 = both.new_state;
    assert!(
        checked_34.is_some(),
        "state_and_delta: combined returned no state"
    );
    let Some(combined) = checked_34 else {
        return;
    };
    assert!(
        combined.as_ref().len() >= base.as_ref().len(),
        "state_and_delta: combined must not shrink base"
    );
}

// no test_usage necessary
