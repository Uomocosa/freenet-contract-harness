use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn assert_validate_rejects_garbage<C: ContractInterface>(cfg: &SuiteConfig) {
    let garbage_states = [Vec::new(), b"not-bincode".to_vec(), vec![0xFF; 32]];
    for bad in garbage_states {
        let result = C::validate_state(
            cfg.params.clone(),
            State::from(bad),
            RelatedContracts::default(),
        );
        assert!(
            result.is_err(),
            "validate_rejects_garbage: garbage state should be rejected"
        );
    }
}

// no test_usage necessary
