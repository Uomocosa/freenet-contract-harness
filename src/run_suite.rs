use freenet_stdlib::prelude::*;

use crate::SuiteConfig;

pub fn run_suite<C: ContractInterface>(cfg: &SuiteConfig) {
    cfg.assert_validate_accepts_gen::<C>();
    cfg.assert_validate_rejects_garbage::<C>();
    cfg.assert_summarize_deterministic::<C>();
    cfg.assert_summarize_detects_structural_divergence::<C>();
    cfg.assert_delta_nonempty_and_roundtrips::<C>();
    cfg.assert_delta_handles_bad_summary::<C>();
    cfg.assert_self_delta_empty::<C>();
    cfg.assert_delta_equivalence::<C>();
    cfg.assert_update_idempotent::<C>();
    cfg.assert_update_commutative::<C>();
    cfg.assert_update_associative::<C>();
    cfg.assert_update_reads_data_not_state_plus1::<C>();
    cfg.assert_update_empty_and_unknown_noop::<C>();
    cfg.assert_update_state_and_delta::<C>();
    cfg.assert_update_rejects_garbage_data_without_panic::<C>();
}

#[cfg(test)]
mod tests {
    use crate::SuiteConfig;
    use crate::run_suite;
    use freenet_stdlib::prelude::*;
    use rstest::rstest;
    use std::collections::BTreeMap;

    struct DummyContract;

    type Slots = BTreeMap<u64, u64>;

    fn decode_slots(state: &[u8]) -> Result<Slots, ContractError> {
        bincode::deserialize::<Slots>(state).map_err(|_| ContractError::InvalidState)
    }

    fn encode_slots(slots: &Slots) -> State<'static> {
        State::from(bincode::serialize(slots).unwrap())
    }

    impl ContractInterface for DummyContract {
        fn validate_state(
            _parameters: Parameters<'static>,
            state: State<'static>,
            _related: RelatedContracts<'static>,
        ) -> Result<ValidateResult, ContractError> {
            decode_slots(state.as_ref()).map(|_| ValidateResult::Valid)
        }

        fn update_state(
            _parameters: Parameters<'static>,
            state: State<'static>,
            data: Vec<UpdateData<'static>>,
        ) -> Result<UpdateModification<'static>, ContractError> {
            let mut current = decode_slots(state.as_ref()).unwrap_or_default();
            for update in data {
                match update {
                    UpdateData::State(s) => {
                        if s.as_ref().is_empty() {
                            continue;
                        }
                        let incoming =
                            decode_slots(s.as_ref()).map_err(|_| ContractError::InvalidUpdate)?;
                        for (tag, value) in incoming {
                            let entry = current.entry(tag).or_insert(0);
                            *entry = (*entry).max(value);
                        }
                    }
                    UpdateData::Delta(d) => {
                        if d.as_ref().is_empty() {
                            continue;
                        }
                        let incoming =
                            decode_slots(d.as_ref()).map_err(|_| ContractError::InvalidUpdate)?;
                        for (tag, value) in incoming {
                            let entry = current.entry(tag).or_insert(0);
                            *entry = (*entry).max(value);
                        }
                    }
                    UpdateData::StateAndDelta { state: s, delta: d } => {
                        if !s.as_ref().is_empty() {
                            let incoming = decode_slots(s.as_ref())
                                .map_err(|_| ContractError::InvalidUpdate)?;
                            for (tag, value) in incoming {
                                let entry = current.entry(tag).or_insert(0);
                                *entry = (*entry).max(value);
                            }
                        }
                        if !d.as_ref().is_empty() {
                            let incoming = decode_slots(d.as_ref())
                                .map_err(|_| ContractError::InvalidUpdate)?;
                            for (tag, value) in incoming {
                                let entry = current.entry(tag).or_insert(0);
                                *entry = (*entry).max(value);
                            }
                        }
                    }
                    _ => {}
                }
            }
            let new_state = encode_slots(&current);
            Ok(UpdateModification::valid(new_state))
        }

        fn summarize_state(
            _parameters: Parameters<'static>,
            state: State<'static>,
        ) -> Result<StateSummary<'static>, ContractError> {
            let slots = decode_slots(state.as_ref())?;
            let summary =
                bincode::serialize(&slots).map_err(|e| ContractError::Deser(e.to_string()))?;
            Ok(StateSummary::from(summary))
        }

        fn get_state_delta(
            _parameters: Parameters<'static>,
            state: State<'static>,
            summary: StateSummary<'static>,
        ) -> Result<StateDelta<'static>, ContractError> {
            let ours = decode_slots(state.as_ref())?;
            let theirs: Slots = match bincode::deserialize(summary.as_ref()) {
                Ok(s) => s,
                Err(_) => return Ok(StateDelta::from(state.as_ref().to_vec())),
            };
            let mut delta = Slots::new();
            for (tag, value) in &ours {
                if theirs.get(tag).copied().unwrap_or(0) < *value {
                    delta.insert(*tag, *value);
                }
            }
            if delta.is_empty() {
                return Ok(StateDelta::from(Vec::new()));
            }
            Ok(StateDelta::from(
                bincode::serialize(&delta).map_err(|e| ContractError::Deser(e.to_string()))?,
            ))
        }
    }

    fn slots(map: &[(u64, u64)]) -> State<'static> {
        let s: Slots = map.iter().copied().collect();
        encode_slots(&s)
    }

    fn cfg() -> SuiteConfig {
        SuiteConfig {
            params: Parameters::from(Vec::new()),
            gen_state: || slots(&[(0, 5), (1, 3)]),
            gen_update: |_| UpdateData::State(slots(&[(1, 7)])),
            gen_divergent_equal_total: Some(|| Some((slots(&[(0, 4), (1, 4)]), slots(&[(0, 8)])))),
            empty_state: || slots(&[]),
        }
    }

    #[rstest]
    #[case::validate_accepts_gen(SuiteConfig::assert_validate_accepts_gen::<DummyContract>)]
    #[case::validate_rejects_garbage(SuiteConfig::assert_validate_rejects_garbage::<DummyContract>)]
    #[case::summarize_deterministic(SuiteConfig::assert_summarize_deterministic::<DummyContract>)]
    #[case::summarize_detects_structural_divergence(
        SuiteConfig::assert_summarize_detects_structural_divergence::<DummyContract>
    )]
    #[case::delta_nonempty_and_roundtrips(
        SuiteConfig::assert_delta_nonempty_and_roundtrips::<DummyContract>
    )]
    #[case::delta_handles_bad_summary(SuiteConfig::assert_delta_handles_bad_summary::<DummyContract>)]
    #[case::update_idempotent(SuiteConfig::assert_update_idempotent::<DummyContract>)]
    #[case::update_commutative(SuiteConfig::assert_update_commutative::<DummyContract>)]
    #[case::update_associative(SuiteConfig::assert_update_associative::<DummyContract>)]
    #[case::update_reads_data_not_state_plus1(
        SuiteConfig::assert_update_reads_data_not_state_plus1::<DummyContract>
    )]
    #[case::update_empty_and_unknown_noop(
        SuiteConfig::assert_update_empty_and_unknown_noop::<DummyContract>
    )]
    #[case::update_rejects_garbage_data_without_panic(
        SuiteConfig::assert_update_rejects_garbage_data_without_panic::<DummyContract>
    )]
    #[case::self_delta_empty(SuiteConfig::assert_self_delta_empty::<DummyContract>)]
    #[case::delta_equivalence(SuiteConfig::assert_delta_equivalence::<DummyContract>)]
    #[case::state_and_delta(SuiteConfig::assert_update_state_and_delta::<DummyContract>)]
    fn contract_check(#[case] check: fn(&SuiteConfig)) {
        check(&cfg());
    }

    #[test]
    fn test_usage() {
        run_suite::run_suite::<DummyContract>(&cfg());
    }
}
