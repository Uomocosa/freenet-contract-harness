use freenet_stdlib::prelude::*;

use crate::testing;

pub fn decode_slots(state: &[u8]) -> Result<testing::Slots, ContractError> {
    bincode::deserialize::<testing::Slots>(state).map_err(|_| ContractError::InvalidState)
}

#[cfg(test)]
mod tests {
    use super::decode_slots;
    use crate::testing::{Slots, encode_slots};

    #[test]
    fn test_usage() {
        let slots: Slots = [(0, 5), (1, 3)].into_iter().collect();
        assert_eq!(decode_slots(encode_slots(&slots).as_ref()).unwrap(), slots);
        assert!(decode_slots(&[0xff]).is_err());
    }
}
