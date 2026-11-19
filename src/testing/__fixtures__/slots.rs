use std::collections::BTreeMap;

pub type Slots = BTreeMap<u64, u64>;

#[cfg(test)]
mod tests {
    use super::Slots;

    #[test]
    fn test_usage() {
        let slots: Slots = [(0, 5), (1, 3)].into_iter().collect();
        assert_eq!(slots.get(&0), Some(&5));
        assert_eq!(slots.len(), 2);
    }
}
