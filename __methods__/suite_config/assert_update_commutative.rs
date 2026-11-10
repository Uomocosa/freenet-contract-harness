use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_update_commutative<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let a = (cfg.gen_update)(&state);
    let b = (cfg.gen_update)(&state);
    let checked_14 = SuiteConfig::try_apply::<C>(
        cfg.params.clone(),
        state.clone(),
        vec![a.clone(), b.clone()],
    );
    assert!(checked_14.is_some(), "commutative ab failed");
    let Some(ab) = checked_14 else {
        return;
    };
    let checked_15 = SuiteConfig::try_apply::<C>(cfg.params.clone(), state, vec![b, a]);
    assert!(checked_15.is_some(), "commutative ba failed");
    let Some(ba) = checked_15 else {
        return;
    };
    assert_eq!(
        ab.as_ref(),
        ba.as_ref(),
        "update_commutative: order of updates must not affect result"
    );
}

// no test_usage necessary
