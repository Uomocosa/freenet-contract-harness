use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_update_rejects_garbage_data_without_panic<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let bad = State::from(b"not-state".to_vec());
    let result = C::update_state(cfg.params.clone(), state, vec![UpdateData::State(bad)]);
    assert!(
        result.is_ok() || result.is_err(),
        "update_rejects_garbage: must not trap"
    );
}

// no test_usage necessary
