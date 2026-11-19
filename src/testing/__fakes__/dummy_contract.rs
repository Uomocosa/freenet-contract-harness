use freenet_stdlib::prelude::*;

use crate::testing::{Slots, decode_slots, encode_slots};

pub struct DummyContract;

#[cfg(test)]
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
                        let incoming =
                            decode_slots(s.as_ref()).map_err(|_| ContractError::InvalidUpdate)?;
                        for (tag, value) in incoming {
                            let entry = current.entry(tag).or_insert(0);
                            *entry = (*entry).max(value);
                        }
                    }
                    if !d.as_ref().is_empty() {
                        let incoming =
                            decode_slots(d.as_ref()).map_err(|_| ContractError::InvalidUpdate)?;
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

#[cfg(test)]
mod tests {
    use super::DummyContract;
    use crate::testing::slots_state;
    use freenet_stdlib::prelude::*;

    #[test]
    fn test_usage() {
        let result = DummyContract::validate_state(
            Parameters::from(Vec::new()),
            slots_state(&[(0, 5)]),
            RelatedContracts::default(),
        );
        assert!(matches!(result, Ok(ValidateResult::Valid)));
    }
}
