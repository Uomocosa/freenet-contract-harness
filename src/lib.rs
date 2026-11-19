#[path = "__basic__/mod.rs"]
mod basic;
#[path = "../__methods__/mod.rs"]
mod methods;
mod run_suite;
mod suite_config;
#[cfg(test)]
pub(crate) mod testing;

pub use basic::type_aliases::DivergentGen;
pub use run_suite::run_suite;
pub use suite_config::SuiteConfig;
