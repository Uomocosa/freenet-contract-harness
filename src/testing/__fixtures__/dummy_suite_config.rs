use freenet_stdlib::prelude::*;

use crate::SuiteConfig;
use crate::testing;

pub fn dummy_suite_config() -> SuiteConfig {
    SuiteConfig {
        params: Parameters::from(Vec::new()),
        gen_state: || testing::slots_state(&[(0, 5), (1, 3)]),
        gen_update: |_| UpdateData::State(testing::slots_state(&[(1, 7)])),
        gen_divergent_equal_total: Some(|| {
            Some((
                testing::slots_state(&[(0, 4), (1, 4)]),
                testing::slots_state(&[(0, 8)]),
            ))
        }),
        empty_state: || testing::slots_state(&[]),
    }
}

#[cfg(test)]
mod tests {
    use super::dummy_suite_config;
    use crate::testing;

    #[test]
    fn test_usage() {
        let config = dummy_suite_config();
        assert_eq!(
            testing::decode_slots((config.gen_state)().as_ref())
                .unwrap()
                .len(),
            2
        );
        assert!(config.gen_divergent_equal_total.is_some());
    }
}
