use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_validate_accepts_gen<C: ContractInterface>(cfg: &SuiteConfig) {
    let state = (cfg.gen_state)();
    let result = C::validate_state(cfg.params.clone(), state, RelatedContracts::default());
    assert!(
        result.is_ok(),
        "validate_accepts_gen: gen_state should be valid"
    );
}

// no test_usage necessary
