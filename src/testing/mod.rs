#[path = "__basic__/mod.rs"]
mod basic;
#[path = "__fakes__/mod.rs"]
pub mod fakes;
#[path = "__fixtures__/mod.rs"]
pub mod fixtures;

pub use basic::type_aliases::Slots;
pub use fakes::dummy_contract::DummyContract;
pub use fixtures::decode_slots::decode_slots;
pub use fixtures::dummy_suite_config::dummy_suite_config;
pub use fixtures::encode_slots::encode_slots;
pub use fixtures::slots_state::slots_state;
