#[path = "__basic__/mod.rs"]
pub mod basic;
mod run_suite;

pub use basic::structs::SuiteConfig;
pub use basic::type_aliases::DivergentGen;
pub use run_suite::run_suite;

// no test_usage necessary
