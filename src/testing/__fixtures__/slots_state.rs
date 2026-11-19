use freenet_stdlib::prelude::*;

use crate::testing::{Slots, encode_slots};

pub fn slots_state(map: &[(u64, u64)]) -> State<'static> {
    let slots: Slots = map.iter().copied().collect();
    encode_slots(&slots)
}

#[cfg(test)]
mod tests {
    use super::slots_state;
    use crate::testing::decode_slots;

    #[test]
    fn test_usage() {
        let state = slots_state(&[(0, 5), (1, 3)]);
        assert_eq!(decode_slots(state.as_ref()).unwrap().len(), 2);
    }
}
