#[path = "__basic__/mod.rs"]
pub mod basic;
#[path = "../methods/mod.rs"]
mod methods;
mod run_suite;
mod suite_config;

pub use basic::type_aliases::DivergentGen;
pub use run_suite::run_suite;
pub use suite_config::SuiteConfig;

// no test_usage necessary
