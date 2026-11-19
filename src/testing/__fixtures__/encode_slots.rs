use freenet_stdlib::prelude::*;

use crate::testing;

pub fn encode_slots(slots: &testing::Slots) -> State<'static> {
    State::from(bincode::serialize(slots).unwrap())
}

#[cfg(test)]
mod tests {
    use super::encode_slots;
    use crate::testing::{Slots, decode_slots};

    #[test]
    fn test_usage() {
        let slots: Slots = [(0, 5), (1, 3)].into_iter().collect();
        assert_eq!(decode_slots(encode_slots(&slots).as_ref()).unwrap(), slots);
    }
}
